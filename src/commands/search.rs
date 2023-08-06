use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::util::search::SearchContext;

use super::cursor_move;
use super::search_glob;
use super::search_string;

pub fn search_next(context: &mut AppContext) -> JoshutoResult {
    if let Some(search_context) = context.get_search_context() {
        let index = match search_context {
            SearchContext::Glob(glob) => {
                search_glob::search_glob_fwd(context.tab_context_ref().curr_tab_ref(), glob)
            }
            SearchContext::String {
                pattern,
                actual_case_sensitivity,
            } => search_string::search_string_fwd(
                context.tab_context_ref().curr_tab_ref(),
                pattern,
                *actual_case_sensitivity,
            ),
        };
        if let Some(index) = index {
            cursor_move::cursor_move(context, index);
        }
    }
    Ok(())
}

pub fn search_prev(context: &mut AppContext) -> JoshutoResult {
    if let Some(search_context) = context.get_search_context() {
        let index = match search_context {
            SearchContext::Glob(glob) => {
                search_glob::search_glob_rev(context.tab_context_ref().curr_tab_ref(), glob)
            }
            SearchContext::String {
                pattern,
                actual_case_sensitivity,
            } => search_string::search_string_rev(
                context.tab_context_ref().curr_tab_ref(),
                pattern,
                *actual_case_sensitivity,
            ),
        };
        if let Some(index) = index {
            cursor_move::cursor_move(context, index);
        }
    }
    Ok(())
}
