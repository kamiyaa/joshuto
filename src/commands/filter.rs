use crate::context::{AppContext, MatchContext};
use crate::error::AppResult;

use super::reload;

pub fn filter(context: &mut AppContext, pattern: &str) -> AppResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .string_case_sensitivity;

    let filter_context = MatchContext::new_string(pattern, case_sensitivity);

    let curr_tab = context.tab_context_mut().curr_tab_mut();
    let path = curr_tab.cwd().to_path_buf();

    curr_tab
        .option_mut()
        .dirlist_options_mut(&path)
        .set_filter_context(filter_context);

    if let Some(list) = curr_tab.curr_list_mut() {
        list.depreciate();
    }

    reload::soft_reload_curr_tab(context)?;
    Ok(())
}
