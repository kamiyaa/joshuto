use std::path;

use crate::commands::cursor_move;
use crate::error::AppResult;
use crate::types::state::AppState;

use super::tab_ops;

pub fn new_directory(app_state: &mut AppState, p: &path::Path) -> AppResult {
    std::fs::create_dir_all(p)?;

    let curr_path = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .get_cwd()
        .to_path_buf();
    tab_ops::reload_all_tabs(app_state, curr_path.as_path())?;

    if app_state.config.focus_on_create {
        cursor_move::to_path(app_state, p)?;
    }

    Ok(())
}
