use std::path::PathBuf;

use crate::error::AppResult;
use crate::types::state::AppState;

pub fn parent_cursor_move(app_state: &mut AppState, new_index: usize) -> AppResult {
    let mut path: Option<PathBuf> = None;
    let mut new_index = new_index;

    {
        let ui_state = app_state.state.ui_state_ref().clone();
        let display_options = app_state.config.display_options.clone();
        let curr_tab = app_state.state.tab_state_mut().curr_tab_mut();
        if let Some(curr_list) = curr_tab.parent_list_mut() {
            if curr_list.get_index().is_some() {
                let dir_len = curr_list.contents.len();
                if new_index >= dir_len {
                    new_index = dir_len - 1;
                }
                let entry = &curr_list.contents[new_index];
                if entry.file_path().is_dir() {
                    path = Some(entry.file_path().to_path_buf());
                    curr_list.set_index(Some(new_index), &ui_state, &display_options);
                }
            }
        }
        if let Some(path) = path.as_ref() {
            curr_tab.set_cwd(path);
        }
    }
    Ok(())
}

pub fn parent_up(app_state: &mut AppState, u: usize) -> AppResult {
    let movement = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .parent_list_ref()
        .and_then(|list| list.get_index().map(|idx| idx.saturating_sub(u)));

    if let Some(s) = movement {
        parent_cursor_move(app_state, s)?;
    }
    app_state.state.update_watcher();
    Ok(())
}

pub fn parent_down(app_state: &mut AppState, u: usize) -> AppResult {
    let movement = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .parent_list_ref()
        .and_then(|list| list.get_index().map(|idx| idx + u));

    if let Some(s) = movement {
        parent_cursor_move(app_state, s)?;
    }
    Ok(())
}
