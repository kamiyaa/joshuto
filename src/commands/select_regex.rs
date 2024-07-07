use crate::error::AppResult;
use crate::types::state::{AppState, MatchState};

use super::select::{self, SelectOption};

pub fn select_regex(app_state: &mut AppState, pattern: &str, options: &SelectOption) -> AppResult {
    let case_sensitivity = app_state.config.search_options.regex_case_sensitivity;

    let select_state = MatchState::new_regex(pattern, case_sensitivity)?;
    select::select_files(app_state, &select_state, options)
}
