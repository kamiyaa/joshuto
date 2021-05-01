use globset::{GlobBuilder, GlobMatcher};

use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::tab::JoshutoTab;
use crate::util::search::SearchPattern;

use super::cursor_move;

pub fn search_glob_fwd(curr_tab: &JoshutoTab, glob: &GlobMatcher) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let offset = curr_list.index? + 1;
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

    let offset = curr_list.index?;
    let contents_len = curr_list.len();
    for i in (0..contents_len).rev() {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();
        if glob.is_match(file_name) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}

pub fn search_glob(context: &mut AppContext, pattern: &str) -> JoshutoResult<()> {
    let glob = match GlobBuilder::new(pattern).case_insensitive(true).build() {
        Ok(s) => s.compile_matcher(),
        Err(e) => {
            return Err(JoshutoError::new(
                JoshutoErrorKind::IoInvalidData,
                "Invalid glob input".to_string(),
            ));
        }
    };

    let index = search_glob_fwd(context.tab_context_ref().curr_tab_ref(), &glob);
    if let Some(index) = index {
        let _ = cursor_move::cursor_move(index, context);
    }
    context.set_search_state(SearchPattern::Glob(glob));
    Ok(())
}
