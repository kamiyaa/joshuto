use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::tab::JoshutoTab;
use crate::util::search::SearchPattern;

use super::cursor_move;

pub fn search_string_fwd(curr_tab: &JoshutoTab, pattern: &str) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let offset = curr_list.index? + 1;
    let contents_len = curr_list.contents.len();
    for i in 0..contents_len {
        let file_name_lower = curr_list.contents[(offset + i) % contents_len]
            .file_name()
            .to_lowercase();
        if file_name_lower.contains(pattern) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}
pub fn search_string_rev(curr_tab: &JoshutoTab, pattern: &str) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let offset = curr_list.index?;
    let contents_len = curr_list.contents.len();
    for i in (0..contents_len).rev() {
        let file_name_lower = curr_list.contents[(offset + i) % contents_len]
            .file_name()
            .to_lowercase();
        if file_name_lower.contains(pattern) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}

pub fn search_string(context: &mut AppContext, pattern: &str) -> JoshutoResult<()> {
    let pattern = pattern.to_lowercase();
    let index = search_string_fwd(context.tab_context_ref().curr_tab_ref(), pattern.as_str());
    if let Some(index) = index {
        let _ = cursor_move::cursor_move(context, index);
    }
    context.set_search_context(SearchPattern::String(pattern));
    Ok(())
}
