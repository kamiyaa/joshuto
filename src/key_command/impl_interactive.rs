use crate::commands::*;
use crate::context::AppContext;

use super::{Command, InteractiveExecute};

impl InteractiveExecute for Command {
    fn interactive_execute(&self, context: &mut AppContext) {
        match self {
            Self::SearchIncremental { pattern } => {
                search_string::search_string(context, pattern.as_str(), true)
            }
            Self::Filter { pattern } => {
                filter::filter(context, pattern).ok();
            }
            _ => (),
        }
    }
}
