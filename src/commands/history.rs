use std::path::PathBuf;

use crate::error::AppResult;
use crate::types::state::AppState;

use super::change_directory;

fn next_or_prev(app_state: &mut AppState, path_opt: Option<PathBuf>) -> AppResult {
    if let Some(path) = path_opt {
        if !matches!(path.try_exists(), Ok(true)) {
            app_state
                .state
                .tab_state_mut()
                .curr_tab_mut()
                .navigation_history_mut()
                .remove_current();
            return next(app_state);
        }

        change_directory::cd(&path, app_state, false)?;
    }

    Ok(())
}

pub fn next(app_state: &mut AppState) -> AppResult {
    let next = app_state
        .state
        .tab_state_mut()
        .curr_tab_mut()
        .navigation_history_mut()
        .next()
        .cloned();
    next_or_prev(app_state, next)
}

pub fn prev(app_state: &mut AppState) -> AppResult {
    let prev = app_state
        .state
        .tab_state_mut()
        .curr_tab_mut()
        .navigation_history_mut()
        .prev()
        .cloned();
    next_or_prev(app_state, prev)
}
