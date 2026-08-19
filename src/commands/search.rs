use crate::error::AppResult;
use crate::tab::JoshutoTab;
use crate::types::state::{AppState, MatchState};

use super::cursor_move;

/// Implements `search_next`: moves the cursor to the next entry matching the active search
/// pattern.
pub fn search_next(app_state: &mut AppState) -> AppResult {
    if let Some(search_state) = app_state.state.get_search_state() {
        if search_state.is_none() {
            return Ok(());
        }

        let curr_tab = &app_state.state.tab_state_ref().curr_tab_ref();
        let index = curr_tab.curr_list_ref().and_then(|c| c.get_index());

        let offset = match index {
            Some(index) => index + 1,
            None => return Ok(()),
        };

        if let Some(index) = search_next_impl(curr_tab, search_state, offset) {
            cursor_move::cursor_move(app_state, index);
        }
    }

    Ok(())
}

/// Returns the index of the next entry (searching forward, wrapping around) matching
/// `match_state`, starting at `offset`.
pub(super) fn search_next_impl(
    curr_tab: &JoshutoTab,
    match_state: &MatchState,
    offset: usize,
) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;
    let contents_len = curr_list.contents.len();

    for i in 0..contents_len {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();

        if match_state.is_match(file_name) {
            return Some((offset + i) % contents_len);
        }
    }

    None
}

/// Implements `search_prev`: moves the cursor to the previous entry matching the active search
/// pattern.
pub fn search_prev(app_state: &mut AppState) -> AppResult {
    if let Some(search_state) = app_state.state.get_search_state() {
        if search_state.is_none() {
            return Ok(());
        }

        let curr_tab = &app_state.state.tab_state_ref().curr_tab_ref();
        let index = curr_tab.curr_list_ref().and_then(|c| c.get_index());

        let offset = match index {
            Some(index) => index,
            None => return Ok(()),
        };

        if let Some(index) = search_prev_impl(curr_tab, search_state, offset) {
            cursor_move::cursor_move(app_state, index);
        }
    }

    Ok(())
}

fn search_prev_impl(
    curr_tab: &JoshutoTab,
    match_state: &MatchState,
    offset: usize,
) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;
    let contents_len = curr_list.contents.len();

    for i in (0..contents_len).rev() {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();

        if match_state.is_match(file_name) {
            return Some((offset + i) % contents_len);
        }
    }

    None
}
