use crate::config::option::SelectOption;
use crate::context::{AppContext, MatchContext};
use crate::error::JoshutoResult;

use super::select;

pub fn select_regex(
    context: &mut AppContext,
    pattern: &str,
    options: &SelectOption,
) -> JoshutoResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .regex_case_sensitivity;

    let select_context = MatchContext::new_regex(pattern, case_sensitivity)?;
    select::select_files(context, &select_context, options)
}
