use crate::context::{AppContext, MatchContext};
use crate::error::AppResult;

use super::cursor_move;
use super::search;

pub fn search_regex(context: &mut AppContext, pattern: &str) -> AppResult {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .regex_case_sensitivity;

    let search_context = MatchContext::new_regex(pattern, case_sensitivity)?;

    let curr_tab = &context.tab_context_ref().curr_tab_ref();
    let index = curr_tab.curr_list_ref().and_then(|c| c.get_index());

    let offset = match index {
        Some(index) => index + 1,
        None => return Ok(()),
    };

    if let Some(new_index) = search::search_next_impl(curr_tab, &search_context, offset) {
        cursor_move::cursor_move(context, new_index);
    }

    context.set_search_context(search_context);
    Ok(())
}
