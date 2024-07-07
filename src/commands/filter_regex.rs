use crate::error::AppResult;
use crate::types::state::{AppState, MatchState};

use super::filter;

pub fn filter_regex(app_state: &mut AppState, pattern: &str) -> AppResult {
    let case_sensitivity = app_state.config.search_options.regex_case_sensitivity;

    let filter_state = MatchState::new_regex(pattern, case_sensitivity)?;
    filter::filter(app_state, filter_state)
}
