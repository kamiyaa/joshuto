use crate::types::state::{AppState, MatchState};

use super::cursor_move;
use super::search;

pub fn search_string(app_state: &mut AppState, pattern: &str, incremental: bool) {
    let case_sensitivity = app_state.config.search_options.string_case_sensitivity;

    let search_state = MatchState::new_string(pattern, case_sensitivity);

    let curr_tab = app_state.state.tab_state_ref().curr_tab_ref();

    if incremental {
        if let Some(new_index) = search::search_next_impl(curr_tab, &search_state, 0) {
            cursor_move::cursor_move(app_state, new_index);
        }
    } else if let Some(index) = curr_tab.curr_list_ref().and_then(|c| c.get_index()) {
        let offset = index + 1;

        if let Some(new_index) = search::search_next_impl(curr_tab, &search_state, offset) {
            cursor_move::cursor_move(app_state, new_index);
        }
    }

    app_state.state.set_search_state(search_state);
}
