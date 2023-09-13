use crate::context::{AppContext, MatchContext};
use crate::error::AppResult;

use super::select::{self, SelectOption};

pub fn select_string(context: &mut AppContext, pattern: &str, options: &SelectOption) -> AppResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .string_case_sensitivity;

    let select_context = if !pattern.is_empty() {
        MatchContext::new_string(pattern, case_sensitivity)
    } else {
        MatchContext::None
    };

    select::select_files(context, &select_context, options)
}
