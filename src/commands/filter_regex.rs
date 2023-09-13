use crate::context::{AppContext, MatchContext};
use crate::error::AppResult;

use super::filter;

pub fn filter_regex(context: &mut AppContext, pattern: &str) -> AppResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .regex_case_sensitivity;

    let filter_context = MatchContext::new_regex(pattern, case_sensitivity)?;
    filter::filter(context, filter_context)
}
