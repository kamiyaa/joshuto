use std::collections::HashMap;
use std::path::Path;
use std::{io, path};

use uuid::Uuid;

use crate::config::clean::app::display::new_tab::NewTabMode;
use crate::context::AppContext;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::history::{
    create_dirlist_with_history, generate_entries_to_root, DirectoryHistory, JoshutoHistory,
};
use crate::tab::{JoshutoTab, TabHomePage};
use crate::util::{cwd, unix};

use crate::HOME_DIR;

use super::quit::{quit_with_action, QuitAction};

fn _tab_switch(new_index: usize, context: &mut AppContext) -> std::io::Result<()> {
    context.tab_context_mut().index = new_index;
    let cwd = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
    cwd::set_current_dir(cwd.as_path())?;

    let entry_path = match context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|l| l.curr_entry_ref())
    {
        Some(entry) => {
            let file_path = entry.file_path();
            if file_path.is_dir() {
                Some(file_path.to_path_buf())
            } else {
                None
            }
        }
        None => None,
    };

    let display_options = context.config_ref().display_options_ref();
    let tab_options = context.tab_context_ref().curr_tab_ref().option_ref();

    let history = context.tab_context_ref().curr_tab_ref().history_ref();

    let mut dirlists = Vec::with_capacity(3);
    for curr_path in [
        Some(cwd.as_path().to_path_buf()),
        cwd.parent().map(|p| p.to_path_buf()),
        entry_path,
    ]
    .into_iter()
    .flatten()
    {
        match history.get(&curr_path) {
            Some(list) => {
                if list.need_update() {
                    let dirlist = create_dirlist_with_history(
                        history,
                        cwd.as_path(),
                        display_options,
                        tab_options,
                    )?;
                    dirlists.push(dirlist);
                }
            }
            None => {
                let dirlist = create_dirlist_with_history(
                    history,
                    cwd.as_path(),
                    display_options,
                    tab_options,
                )?;
                dirlists.push(dirlist);
            }
        }
    }

    let history = context.tab_context_mut().curr_tab_mut().history_mut();
    history.insert_entries(dirlists);

    Ok(())
}

pub fn tab_switch(context: &mut AppContext, offset: i32) -> std::io::Result<()> {
    let index = context.tab_context_ref().index;
    let num_tabs = context.tab_context_ref().len();
    let new_index = (index as i32 + num_tabs as i32 + offset) as usize % num_tabs;

    _tab_switch(new_index, context)
}

pub fn tab_switch_index(context: &mut AppContext, new_index: usize) -> AppResult {
    let num_tabs = context.tab_context_ref().len();
    if new_index <= num_tabs {
        _tab_switch(new_index - 1, context)?;
    } else if new_index > num_tabs {
        for _ in 0..(new_index - num_tabs) {
            new_tab(context, &NewTabMode::Default, true)?;
        }
        _tab_switch(new_index - 1, context)?;
    }
    Ok(())
}

pub fn new_tab_home_path(context: &AppContext) -> path::PathBuf {
    match context.config_ref().tab_options_ref().home_page() {
        TabHomePage::Home => match HOME_DIR.as_ref() {
            Some(s) => s.clone(),
            None => path::PathBuf::from("/"),
        },
        TabHomePage::Inherit => context.tab_context_ref().curr_tab_ref().cwd().to_path_buf(),
        TabHomePage::Root => path::PathBuf::from("/"),
    }
}

pub fn new_tab(context: &mut AppContext, mode: &NewTabMode, last: bool) -> AppResult {
    let new_tab_path = match mode {
        NewTabMode::Default => Ok(new_tab_home_path(context)),
        NewTabMode::CurrentTabDir => {
            Ok(context.tab_context_ref().curr_tab_ref().cwd().to_path_buf())
        }
        NewTabMode::CursorDir => context
            .tab_context_ref()
            .curr_tab_ref()
            .curr_list_ref()
            .and_then(|list| {
                list.curr_entry_ref().and_then(|entry| {
                    if entry.metadata.is_dir() {
                        Some(entry.file_path_buf())
                    } else {
                        None
                    }
                })
            })
            .ok_or(AppError::new(
                AppErrorKind::InvalidParameters,
                "No directory at cursor.".to_string(),
            )),
        NewTabMode::Directory(directory) => {
            let directory_path = unix::expand_shell_string(directory);
            Ok(if directory_path.is_absolute() {
                directory_path
            } else {
                let mut tab_dir = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
                tab_dir.push(directory_path);
                tab_dir
            })
        }
    }?;
    if new_tab_path.exists() && new_tab_path.is_dir() {
        let id = Uuid::new_v4();
        let mut new_tab_history = JoshutoHistory::new();
        let tab_display_options = context
            .config_ref()
            .display_options_ref()
            .default_tab_display_option
            .clone();
        let dirlists = generate_entries_to_root(
            new_tab_path.as_path(),
            &new_tab_history,
            context.ui_context_ref(),
            context.config_ref().display_options_ref(),
            &tab_display_options,
        )?;
        new_tab_history.insert_entries(dirlists);

        let tab_display_options = context
            .config_ref()
            .display_options_ref()
            .default_tab_display_option
            .clone();
        let tab = JoshutoTab::new(new_tab_path, new_tab_history, tab_display_options)?;
        context.tab_context_mut().insert_tab(id, tab, last);
        let new_index = if last {
            context.tab_context_ref().len() - 1
        } else {
            context.tab_context_ref().index + 1
        };

        context.tab_context_mut().index = new_index;
        _tab_switch(new_index, context)?;
        Ok(())
    } else {
        AppResult::Err(AppError::new(
            AppErrorKind::InvalidParameters,
            "Directory does not exist.".to_string(),
        ))
    }
}

pub fn close_tab(context: &mut AppContext) -> AppResult {
    if context.tab_context_ref().len() <= 1 {
        let action = if context.args.change_directory {
            QuitAction::OutputCurrentDirectory
        } else {
            QuitAction::Noop
        };
        return quit_with_action(context, action);
    }
    let curr_tab_id = context.tab_context_ref().curr_tab_id();
    let mut tab_index = context.tab_context_ref().index;

    let _ = context.tab_context_mut().remove_tab(&curr_tab_id);
    let num_tabs = context.tab_context_ref().len();
    if tab_index >= num_tabs {
        tab_index = num_tabs - 1;
    }
    _tab_switch(tab_index, context)?;
    Ok(())
}

pub fn reload_all_tabs(context: &mut AppContext, curr_path: &Path) -> io::Result<()> {
    let mut map = HashMap::new();
    {
        let display_options = context.config_ref().display_options_ref();

        for (id, tab) in context.tab_context_ref().iter() {
            let tab_options = tab.option_ref();
            let history = tab.history_ref();
            let dirlist =
                create_dirlist_with_history(history, curr_path, display_options, tab_options)?;
            map.insert(*id, dirlist);
        }
    }

    for (id, dirlist) in map {
        if let Some(tab) = context.tab_context_mut().tab_mut(&id) {
            tab.history_mut().insert(curr_path.to_path_buf(), dirlist);
        }
    }
    Ok(())
}

pub fn remove_entry_from_all_tabs(context: &mut AppContext, curr_path: &Path) {
    for (_, tab) in context.tab_context_mut().iter_mut() {
        tab.history_mut().remove(curr_path);
    }
}
