use crate::context::{AppContext, MatchContext};
use crate::error::JoshutoResult;

use super::filter;

pub fn filter_glob(context: &mut AppContext, pattern: &str) -> JoshutoResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .glob_case_sensitivity;

    let filter_context = MatchContext::new_glob(pattern, case_sensitivity)?;
    filter::filter(context, filter_context)
}
