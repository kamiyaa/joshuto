use crate::commands::*;
use crate::context::AppContext;

use super::{Command, InteractiveExecute};

impl InteractiveExecute for Command {
    fn interactive_execute(&self, context: &mut AppContext) {
        match self {
            Self::SearchIncremental { pattern } => {
                search_string::search_string(context, pattern.as_str(), true)
            }
            Self::FilterGlob { pattern } => {
                filter_glob::filter_glob(context, pattern).ok();
            }
            Self::FilterRegex { pattern } => {
                filter_regex::filter_regex(context, pattern).ok();
            }
            Self::FilterString { pattern } => {
                filter_string::filter_string(context, pattern).ok();
            }
            _ => (),
        }
    }
}
