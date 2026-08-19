use crate::error::AppResult;
use crate::types::state::{AppState, MatchState};

use super::filter;

/// Implements `filter`: filters the current directory listing by a plain substring.
pub fn filter_string(app_state: &mut AppState, pattern: &str) -> AppResult {
    let case_sensitivity = app_state.config.search_options.string_case_sensitivity;

    let filter_state = MatchState::new_string(pattern, case_sensitivity);
    filter::filter(app_state, filter_state)
}
