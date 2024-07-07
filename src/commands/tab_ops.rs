use std::collections::HashMap;
use std::path::Path;
use std::{io, path};

use uuid::Uuid;

use crate::error::{AppError, AppErrorKind, AppResult};
use crate::history::{
    create_dirlist_with_history, generate_entries_to_root, DirectoryHistory, JoshutoHistory,
};
use crate::tab::{JoshutoTab, NewTabMode, TabHomePage};
use crate::types::state::AppState;
use crate::utils::{cwd, unix};

use crate::HOME_DIR;

use super::quit::{quit_with_action, QuitAction};

fn _tab_switch(new_index: usize, app_state: &mut AppState) -> std::io::Result<()> {
    app_state.state.tab_state_mut().index = new_index;
    let cwd = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .get_cwd()
        .to_path_buf();
    cwd::set_current_dir(cwd.as_path())?;

    let entry_path = match app_state
        .state
        .tab_state_ref()
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

    let display_options = &app_state.config.display_options;
    let tab_options = app_state.state.tab_state_ref().curr_tab_ref().option_ref();

    let history = app_state.state.tab_state_ref().curr_tab_ref().history_ref();

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

    let history = app_state.state.tab_state_mut().curr_tab_mut().history_mut();
    history.insert_entries(dirlists);

    Ok(())
}

pub fn tab_switch(app_state: &mut AppState, offset: i32) -> std::io::Result<()> {
    let index = app_state.state.tab_state_ref().index;
    let num_tabs = app_state.state.tab_state_ref().len();
    let new_index = (index as i32 + num_tabs as i32 + offset) as usize % num_tabs;

    _tab_switch(new_index, app_state)
}

pub fn tab_switch_index(app_state: &mut AppState, new_index: usize) -> AppResult {
    let num_tabs = app_state.state.tab_state_ref().len();
    if new_index <= num_tabs {
        _tab_switch(new_index - 1, app_state)?;
    } else if new_index > num_tabs {
        for _ in 0..(new_index - num_tabs) {
            new_tab(app_state, &NewTabMode::Default, true)?;
        }
        _tab_switch(new_index - 1, app_state)?;
    }
    Ok(())
}

pub fn new_tab_home_path(app_state: &AppState) -> path::PathBuf {
    match app_state.config.tab_options.home_page {
        TabHomePage::Home => match HOME_DIR.as_ref() {
            Some(s) => s.clone(),
            None => path::PathBuf::from("/"),
        },
        TabHomePage::Inherit => app_state
            .state
            .tab_state_ref()
            .curr_tab_ref()
            .get_cwd()
            .to_path_buf(),
        TabHomePage::Root => path::PathBuf::from("/"),
    }
}

pub fn new_tab(app_state: &mut AppState, mode: &NewTabMode, last: bool) -> AppResult {
    let new_tab_path = match mode {
        NewTabMode::Default => Ok(new_tab_home_path(app_state)),
        NewTabMode::CurrentTabDir => Ok(app_state
            .state
            .tab_state_ref()
            .curr_tab_ref()
            .get_cwd()
            .to_path_buf()),
        NewTabMode::CursorDir => app_state
            .state
            .tab_state_ref()
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
                let mut tab_dir = app_state
                    .state
                    .tab_state_ref()
                    .curr_tab_ref()
                    .get_cwd()
                    .to_path_buf();
                tab_dir.push(directory_path);
                tab_dir
            })
        }
    }?;
    if new_tab_path.exists() && new_tab_path.is_dir() {
        let id = Uuid::new_v4();
        let mut new_tab_history = JoshutoHistory::new();
        let tab_display_options = app_state
            .config
            .display_options
            .default_tab_display_option
            .clone();
        let dirlists = generate_entries_to_root(
            new_tab_path.as_path(),
            &new_tab_history,
            app_state.state.ui_state_ref(),
            &app_state.config.display_options,
            &tab_display_options,
        )?;
        new_tab_history.insert_entries(dirlists);

        let tab_display_options = app_state
            .config
            .display_options
            .default_tab_display_option
            .clone();
        let tab = JoshutoTab::new(new_tab_path, new_tab_history, tab_display_options)?;
        app_state.state.tab_state_mut().insert_tab(id, tab, last);
        let new_index = if last {
            app_state.state.tab_state_ref().len() - 1
        } else {
            app_state.state.tab_state_ref().index + 1
        };

        app_state.state.tab_state_mut().index = new_index;
        _tab_switch(new_index, app_state)?;
        Ok(())
    } else {
        AppResult::Err(AppError::new(
            AppErrorKind::InvalidParameters,
            "Directory does not exist.".to_string(),
        ))
    }
}

pub fn close_tab(app_state: &mut AppState) -> AppResult {
    if app_state.state.tab_state_ref().len() <= 1 {
        let action = if app_state.args.change_directory {
            QuitAction::OutputCurrentDirectory
        } else {
            QuitAction::Noop
        };
        return quit_with_action(app_state, action);
    }
    let curr_tab_id = app_state.state.tab_state_ref().curr_tab_id();
    let mut tab_index = app_state.state.tab_state_ref().index;

    let _ = app_state.state.tab_state_mut().remove_tab(&curr_tab_id);
    let num_tabs = app_state.state.tab_state_ref().len();
    if tab_index >= num_tabs {
        tab_index = num_tabs - 1;
    }
    _tab_switch(tab_index, app_state)?;
    Ok(())
}

pub fn reload_all_tabs(app_state: &mut AppState, curr_path: &Path) -> io::Result<()> {
    let mut map = HashMap::new();
    {
        let display_options = &app_state.config.display_options;

        for (id, tab) in app_state.state.tab_state_ref().iter() {
            let tab_options = tab.option_ref();
            let history = tab.history_ref();
            let dirlist =
                create_dirlist_with_history(history, curr_path, display_options, tab_options)?;
            map.insert(*id, dirlist);
        }
    }

    for (id, dirlist) in map {
        if let Some(tab) = app_state.state.tab_state_mut().tab_mut(&id) {
            tab.history_mut().insert(curr_path.to_path_buf(), dirlist);
        }
    }
    Ok(())
}

pub fn remove_entry_from_all_tabs(app_state: &mut AppState, curr_path: &Path) {
    for (_, tab) in app_state.state.tab_state_mut().iter_mut() {
        tab.history_mut().remove(curr_path);
    }
}
