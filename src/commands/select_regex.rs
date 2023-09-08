use crate::context::{AppContext, MatchContext};
use crate::error::AppResult;

use super::select::{self, SelectOption};

pub fn select_regex(context: &mut AppContext, pattern: &str, options: &SelectOption) -> AppResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .regex_case_sensitivity;

    let select_context = MatchContext::new_regex(pattern, case_sensitivity)?;
    select::select_files(context, &select_context, options)
}
