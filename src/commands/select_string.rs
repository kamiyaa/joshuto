use crate::error::AppResult;
use crate::types::state::{AppState, MatchState};

use super::select::{self, SelectOption};

pub fn select_string(app_state: &mut AppState, pattern: &str, options: &SelectOption) -> AppResult {
    let case_sensitivity = app_state.config.search_options.string_case_sensitivity;

    let select_state = if !pattern.is_empty() {
        MatchState::new_string(pattern, case_sensitivity)
    } else {
        MatchState::None
    };

    select::select_files(app_state, &select_state, options)
}
