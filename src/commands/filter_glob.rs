use crate::error::AppResult;
use crate::types::state::{AppState, MatchState};

use super::filter;

/// Implements `filter_glob`: filters the current directory listing by a glob pattern.
pub fn filter_glob(app_state: &mut AppState, pattern: &str) -> AppResult {
    let case_sensitivity = app_state.config.search_options.glob_case_sensitivity;

    let filter_state = MatchState::new_glob(pattern, case_sensitivity)?;
    filter::filter(app_state, filter_state)
}
