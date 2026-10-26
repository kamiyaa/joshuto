use super::commands::*;
use crate::{app_command::AppCommand, state::AppState};

/// Implemented by commands that run interactively, reading further input as they execute.
pub trait InteractiveExecute {
    /// Runs the command's interactive loop against `app_state`.
    fn interactive_execute(&self, app_state: &mut AppState);
}

impl InteractiveExecute for AppCommand {
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
