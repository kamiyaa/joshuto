use crate::context::{AppContext, MatchContext};
use crate::error::AppResult;

use super::select::{self, SelectOption};

pub fn select_glob(context: &mut AppContext, pattern: &str, options: &SelectOption) -> AppResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .glob_case_sensitivity;

    let select_context = MatchContext::new_glob(pattern, case_sensitivity)?;
    select::select_files(context, &select_context, options)
}
