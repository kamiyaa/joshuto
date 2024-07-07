use crate::error::AppResult;
use crate::types::state::{AppState, MatchState};

use super::select::{self, SelectOption};

pub fn select_glob(app_state: &mut AppState, pattern: &str, options: &SelectOption) -> AppResult {
    let case_sensitivity = app_state.config.search_options.glob_case_sensitivity;

    let select_state = MatchState::new_glob(pattern, case_sensitivity)?;
    select::select_files(app_state, &select_state, options)
}
