use crate::error::AppResult;
use crate::types::state::{AppState, MatchState};

use super::cursor_move;
use super::search;

pub fn search_glob(app_state: &mut AppState, pattern: &str) -> AppResult {
    let case_sensitivity = app_state.config.search_options.glob_case_sensitivity;

    let search_state = MatchState::new_glob(pattern, case_sensitivity)?;

    let curr_tab = &app_state.state.tab_state_ref().curr_tab_ref();
    let index = curr_tab.curr_list_ref().and_then(|c| c.get_index());

    let offset = match index {
        Some(index) => index + 1,
        None => return Ok(()),
    };

    if let Some(new_index) = search::search_next_impl(curr_tab, &search_state, offset) {
        cursor_move::cursor_move(app_state, new_index);
    }

    app_state.state.set_search_state(search_state);
    Ok(())
}
