use crate::context::{AppContext, MatchContext};
use crate::error::AppResult;
use crate::tab::JoshutoTab;

use super::cursor_move;

pub fn search_next(context: &mut AppContext) -> AppResult {
    if let Some(search_context) = context.get_search_context() {
        if search_context.is_none() {
            return Ok(());
        }

        let curr_tab = &context.tab_context_ref().curr_tab_ref();
        let index = curr_tab.curr_list_ref().and_then(|c| c.get_index());

        let offset = match index {
            Some(index) => index + 1,
            None => return Ok(()),
        };

        if let Some(index) = search_next_impl(curr_tab, search_context, offset) {
            cursor_move::cursor_move(context, index);
        }
    }

    Ok(())
}

pub(super) fn search_next_impl(
    curr_tab: &JoshutoTab,
    match_context: &MatchContext,
    offset: usize,
) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;
    let contents_len = curr_list.contents.len();

    for i in 0..contents_len {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();

        if match_context.is_match(file_name) {
            return Some((offset + i) % contents_len);
        }
    }

    None
}

pub fn search_prev(context: &mut AppContext) -> AppResult {
    if let Some(search_context) = context.get_search_context() {
        if search_context.is_none() {
            return Ok(());
        }

        let curr_tab = &context.tab_context_ref().curr_tab_ref();
        let index = curr_tab.curr_list_ref().and_then(|c| c.get_index());

        let offset = match index {
            Some(index) => index,
            None => return Ok(()),
        };

        if let Some(index) = search_prev_impl(curr_tab, search_context, offset) {
            cursor_move::cursor_move(context, index);
        }
    }

    Ok(())
}

fn search_prev_impl(
    curr_tab: &JoshutoTab,
    match_context: &MatchContext,
    offset: usize,
) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;
    let contents_len = curr_list.contents.len();

    for i in (0..contents_len).rev() {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();

        if match_context.is_match(file_name) {
            return Some((offset + i) % contents_len);
        }
    }

    None
}
