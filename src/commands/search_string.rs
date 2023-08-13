use crate::context::{AppContext, MatchContext};

use super::cursor_move;
use super::search;

pub fn search_string(context: &mut AppContext, pattern: &str, incremental: bool) {
    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .string_case_sensitivity;

    let search_context = MatchContext::new_string(pattern, case_sensitivity);

    let curr_tab = context.tab_context_ref().curr_tab_ref();

    if incremental {
        if let Some(new_index) = search::search_next_impl(curr_tab, &search_context, 0) {
            cursor_move::cursor_move(context, new_index);
        }
    } else if let Some(index) = curr_tab.curr_list_ref().and_then(|c| c.get_index()) {
        let offset = index + 1;

        if let Some(new_index) = search::search_next_impl(curr_tab, &search_context, offset) {
            cursor_move::cursor_move(context, new_index);
        }
    }

    context.set_search_context(search_context);
}
