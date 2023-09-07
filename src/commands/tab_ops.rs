use std::path;

use uuid::Uuid;

use crate::config::clean::app::display::new_tab::NewTabMode;
use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::history::DirectoryHistory;
use crate::tab::{JoshutoTab, TabHomePage};
use crate::util::unix;

use crate::HOME_DIR;

use super::quit::{quit_with_action, QuitAction};

fn _tab_switch(new_index: usize, context: &mut AppContext) -> std::io::Result<()> {
    context.tab_context_mut().index = new_index;
    let cwd = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
    std::env::set_current_dir(cwd.as_path())?;

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

    let options = context.config_ref().display_options_ref().clone();
    let tab_options = context
        .tab_context_ref()
        .curr_tab_ref()
        .option_ref()
        .clone();

    let history = context.tab_context_mut().curr_tab_mut().history_mut();
    if history
        .create_or_soft_update(cwd.as_path(), &options, &tab_options)
        .is_err()
    {
        history.remove(cwd.as_path());
    }

    if let Some(cwd_parent) = cwd.parent() {
        if history
            .create_or_soft_update(cwd_parent, &options, &tab_options)
            .is_err()
        {
            history.remove(cwd_parent);
        }
    }

    if let Some(file_path) = entry_path {
        if history
            .create_or_soft_update(file_path.as_path(), &options, &tab_options)
            .is_err()
        {
            history.remove(file_path.as_path());
        }
    }

    Ok(())
}

pub fn tab_switch(context: &mut AppContext, offset: i32) -> std::io::Result<()> {
    let index = context.tab_context_ref().index;
    let num_tabs = context.tab_context_ref().len();
    let new_index = (index as i32 + num_tabs as i32 + offset) as usize % num_tabs;

    _tab_switch(new_index, context)
}

pub fn tab_switch_index(context: &mut AppContext, new_index: usize) -> JoshutoResult {
    let num_tabs = context.tab_context_ref().len();
    if new_index <= num_tabs {
        _tab_switch(new_index - 1, context)?;
    } else if new_index > num_tabs {
        for _ in 0..(new_index - num_tabs) {
            new_tab(context, &NewTabMode::Default)?;
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

pub fn new_tab(context: &mut AppContext, mode: &NewTabMode) -> JoshutoResult {
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
            .ok_or(JoshutoError::new(
                JoshutoErrorKind::InvalidParameters,
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
        let tab = JoshutoTab::new(
            new_tab_path,
            context.ui_context_ref(),
            context.config_ref().display_options_ref(),
        )?;
        context.tab_context_mut().insert_tab(id, tab);
        let new_index = context.tab_context_ref().len() - 1;
        context.tab_context_mut().index = new_index;
        _tab_switch(new_index, context)?;
        Ok(())
    } else {
        JoshutoResult::Err(JoshutoError::new(
            JoshutoErrorKind::InvalidParameters,
            "Directory does not exist.".to_string(),
        ))
    }
}

pub fn close_tab(context: &mut AppContext) -> JoshutoResult {
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
