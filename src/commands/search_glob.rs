use globset::{GlobBuilder, GlobMatcher};

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::tab::JoshutoTab;
use crate::util::search::SearchPattern;

use super::cursor_move;

pub fn search_glob_fwd(curr_tab: &JoshutoTab, glob: &GlobMatcher) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let offset = curr_list.get_index()? + 1;
    let contents_len = curr_list.len();
    for i in 0..contents_len {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();
        if glob.is_match(file_name) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}
pub fn search_glob_rev(curr_tab: &JoshutoTab, glob: &GlobMatcher) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let offset = curr_list.get_index()?;
    let contents_len = curr_list.len();
    for i in (0..contents_len).rev() {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();
        if glob.is_match(file_name) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}

pub fn search_glob(context: &mut AppContext, pattern: &str) -> JoshutoResult {
    let glob = GlobBuilder::new(pattern)
        .case_insensitive(true)
        .build()?
        .compile_matcher();

    let index = search_glob_fwd(context.tab_context_ref().curr_tab_ref(), &glob);
    if let Some(index) = index {
        let _ = cursor_move::cursor_move(context, index);
    }
    context.set_search_context(SearchPattern::Glob(glob));
    Ok(())
}
