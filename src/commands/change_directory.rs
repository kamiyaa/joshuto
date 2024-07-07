use std::path;

use crate::commands::{reload, zoxide};
use crate::error::AppResult;
use crate::history::{generate_entries_to_root, DirectoryHistory};
use crate::types::state::AppState;
use crate::utils::cwd;

// ChangeDirectory command
pub fn cd(path: &path::Path, app_state: &mut AppState) -> std::io::Result<()> {
    cwd::set_current_dir(path)?;
    app_state.state.tab_state_mut().curr_tab_mut().set_cwd(path);
    if app_state.config.zoxide_update {
        debug_assert!(path.is_absolute());
        zoxide::zoxide_add(path.to_str().expect("cannot convert path to string"))?;
    }
    Ok(())
}

pub fn change_directory(app_state: &mut AppState, mut path: &path::Path) -> AppResult {
    let new_cwd = if path.is_absolute() {
        path.to_path_buf()
    } else {
        while let Ok(p) = path.strip_prefix("../") {
            parent_directory(app_state)?;
            path = p;
        }

        let mut new_cwd = std::env::current_dir()?;
        new_cwd.push(path);
        new_cwd
    };

    cd(new_cwd.as_path(), app_state)?;
    let dirlists = generate_entries_to_root(
        new_cwd.as_path(),
        app_state.state.tab_state_ref().curr_tab_ref().history_ref(),
        app_state.state.ui_state_ref(),
        &app_state.config.display_options,
        app_state.state.tab_state_ref().curr_tab_ref().option_ref(),
    )?;
    app_state
        .state
        .tab_state_mut()
        .curr_tab_mut()
        .history_mut()
        .insert_entries(dirlists);
    Ok(())
}

// ParentDirectory command
pub fn parent_directory(app_state: &mut AppState) -> AppResult {
    if let Some(parent) = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .get_cwd()
        .parent()
        .map(|p| p.to_path_buf())
    {
        cwd::set_current_dir(&parent)?;
        app_state
            .state
            .tab_state_mut()
            .curr_tab_mut()
            .set_cwd(parent.as_path());
        reload::soft_reload_curr_tab(app_state)?;
    }
    Ok(())
}

// PreviousDirectory command
pub fn previous_directory(app_state: &mut AppState) -> AppResult {
    if let Some(path) = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .previous_dir()
    {
        let path = path.to_path_buf();
        cwd::set_current_dir(&path)?;
        app_state
            .state
            .tab_state_mut()
            .curr_tab_mut()
            .set_cwd(path.as_path());
        reload::soft_reload_curr_tab(app_state)?;
    }
    Ok(())
}
