use std::collections::HashMap;
use std::path::Path;
use std::{io, path};

use uuid::Uuid;

use crate::config::TabHomePage;
use crate::error::{AppError, AppErrorCode, AppResult};
use crate::fs::{
    create_dirlist_with_history, generate_entries_to_root, AppDirectoryHistory, DirectoryHistory,
};
use crate::state::AppState;
use crate::tab::{JoshutoTab, NewTabMode};
use crate::tasks::preview::preview_dir;
use crate::tasks::PreviewDirState;
use crate::utils::{set_current_dir, unix_helper};

use crate::global::HOME_DIR;

use super::quit::{quit_with_action, QuitAction};

/// Makes the tab at `new_index` active: syncs the process cwd to it and refreshes its current,
/// parent, and cursor-entry directories in the background. The cached listings are shown
/// meanwhile, so switching never waits on disk IO (which can take seconds on a spun-down disk).
fn _tab_switch(new_index: usize, app_state: &mut AppState) -> AppResult {
    app_state.state.tab_state_mut().index = new_index;
    let cwd = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .get_cwd()
        .to_path_buf();
    set_current_dir(cwd.as_path())?;

    // judged by cached metadata, which already follows symlinks
    let entry_path = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|l| l.curr_entry_ref())
        .filter(|entry| entry.metadata.is_dir())
        .map(|entry| entry.file_path().to_path_buf());

    for dir_path in [
        Some(cwd.clone()),
        cwd.parent().map(|p| p.to_path_buf()),
        entry_path,
    ]
    .into_iter()
    .flatten()
    {
        let already_loading = matches!(
            app_state
                .state
                .tab_state_ref()
                .curr_tab_ref()
                .history_metadata_ref()
                .get(dir_path.as_path()),
            Some(PreviewDirState::Loading)
        );
        if !already_loading {
            preview_dir::Background::refresh(app_state, dir_path);
        }
    }

    Ok(())
}

/// Implements `tab_switch`: switches to the tab `offset` positions away, wrapping around.
pub fn tab_switch(app_state: &mut AppState, offset: i32) -> AppResult {
    let index = app_state.state.tab_state_ref().index;
    let num_tabs = app_state.state.tab_state_ref().len();
    let new_index = (index as i32 + num_tabs as i32 + offset) as usize % num_tabs;

    _tab_switch(new_index, app_state).map_err(|err| {
        let error_msg = "Failed to switch tabs";
        tracing::error!(?err, "{error_msg}");
        AppError::new(err.code, error_msg.to_string())
    })
}

/// Implements `tab_switch_index`: switches to the 1-indexed tab `new_index`, creating new
/// default tabs to fill the gap if it doesn't exist yet.
pub fn tab_switch_index(app_state: &mut AppState, new_index: usize) -> AppResult {
    if new_index == 0 {
        return Err(AppError::new(
            AppErrorCode::InvalidParameters,
            "Tab index starts at 1".to_string(),
        ));
    }
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

/// Resolves the configured [`TabHomePage`] to a concrete path for a new tab.
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

/// Implements `new_tab`: opens a new tab at the directory specified by `mode`, placing it last
/// or right after the current tab.
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
                AppErrorCode::InvalidParameters,
                "No directory at cursor.".to_string(),
            )),
        NewTabMode::Directory(directory) => {
            let directory_path = unix_helper::expand_shell_string(directory);
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
        let mut new_tab_history = AppDirectoryHistory::new();
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
            AppErrorCode::InvalidParameters,
            "Directory does not exist.".to_string(),
        ))
    }
}

/// Implements `close_tab`: closes the current tab, or quits joshuto if it's the last tab.
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

/// Re-reads `curr_path` from disk into every open tab's listing cache.
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

/// Removes `curr_path` from every open tab's listing cache (e.g. after it's deleted).
pub fn remove_entry_from_all_tabs(app_state: &mut AppState, curr_path: &Path) {
    for (_, tab) in app_state.state.tab_state_mut().iter_mut() {
        tab.history_mut().remove(curr_path);
    }
}
