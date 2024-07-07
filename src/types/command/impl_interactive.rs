use crate::commands::*;

use crate::traits::app_execute::InteractiveExecute;
use crate::types::state::AppState;

use super::Command;

impl InteractiveExecute for Command {
    fn interactive_execute(&self, app_state: &mut AppState) {
        match self {
            Self::SearchIncremental { pattern } => {
                search_string::search_string(app_state, pattern.as_str(), true)
            }
            Self::FilterGlob { pattern } => {
                filter_glob::filter_glob(app_state, pattern).ok();
            }
            Self::FilterRegex { pattern } => {
                filter_regex::filter_regex(app_state, pattern).ok();
            }
            Self::FilterString { pattern } => {
                filter_string::filter_string(app_state, pattern).ok();
            }
            _ => (),
        }
    }
}
