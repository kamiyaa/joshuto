use crate::context::{AppContext, MatchContext};
use crate::error::AppResult;

use super::filter;

pub fn filter_glob(context: &mut AppContext, pattern: &str) -> AppResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .glob_case_sensitivity;

    let filter_context = MatchContext::new_glob(pattern, case_sensitivity)?;
    filter::filter(context, filter_context)
}
