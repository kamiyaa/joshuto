use globset::{GlobBuilder, GlobMatcher};

use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::tab::JoshutoTab;

use super::cursor_move;

pub fn _search(curr_tab: &JoshutoTab, glob: &GlobMatcher) -> Option<usize> {
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
pub fn _search_rev(curr_tab: &JoshutoTab, glob: &GlobMatcher) -> Option<usize> {
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

pub fn search(context: &mut JoshutoContext, pattern: &str) -> JoshutoResult<()> {
    let pattern = pattern.to_lowercase();

    let glob = match GlobBuilder::new(pattern.as_str())
        .case_insensitive(true)
        .build()
    {
        Ok(s) => s.compile_matcher(),
        Err(e) => {
            return Err(JoshutoError::new(
                JoshutoErrorKind::IoInvalidData,
                "Invalid glob input".to_string(),
            ));
        }
    };

    let index = _search(context.tab_context_ref().curr_tab_ref(), &glob);
    if let Some(index) = index {
        let _ = cursor_move::cursor_move(index, context);
    }
    context.set_search_state(glob);
    Ok(())
}

fn search_with_func(
    context: &mut JoshutoContext,
    search_func: fn(&JoshutoTab, &GlobMatcher) -> Option<usize>,
) {
    if let Some(s) = context.get_search_state() {
        let index = search_func(context.tab_context_ref().curr_tab_ref(), s);
        if let Some(index) = index {
            let _ = cursor_move::cursor_move(index, context);
        }
    }
}

pub fn search_next(context: &mut JoshutoContext) -> JoshutoResult<()> {
    search_with_func(context, _search);
    Ok(())
}

pub fn search_prev(context: &mut JoshutoContext) -> JoshutoResult<()> {
    search_with_func(context, _search_rev);
    Ok(())
}
