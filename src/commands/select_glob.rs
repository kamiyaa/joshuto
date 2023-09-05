use crate::config::option::SelectOption;
use crate::context::{AppContext, MatchContext};
use crate::error::JoshutoResult;

use super::select;

pub fn select_glob(
    context: &mut AppContext,
    pattern: &str,
    options: &SelectOption,
) -> JoshutoResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .glob_case_sensitivity;

    let select_context = MatchContext::new_glob(pattern, case_sensitivity)?;
    select::select_files(context, &select_context, options)
}
