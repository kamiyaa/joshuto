use crate::context::{AppContext, MatchContext};
use crate::error::JoshutoResult;

use super::filter;

pub fn filter_string(context: &mut AppContext, pattern: &str) -> JoshutoResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .string_case_sensitivity;

    let filter_context = MatchContext::new_string(pattern, case_sensitivity);
    filter::filter(context, filter_context)
}
