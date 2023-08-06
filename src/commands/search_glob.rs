use globset::{GlobBuilder, GlobMatcher};

use crate::config::option::CaseSensitivity;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::tab::JoshutoTab;
use crate::util::search::SearchContext;

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
    let case_sensitivity = context.config_ref().search_options_ref().case_sensitivity;
    let pattern_lower = pattern.to_lowercase();

    let (pattern, actual_case_sensitivity) = match case_sensitivity {
        CaseSensitivity::Insensitive => (pattern_lower.as_str(), CaseSensitivity::Insensitive),
        CaseSensitivity::Sensitive => (pattern, CaseSensitivity::Sensitive),
        // Determine the actual case sensitivity by whether an uppercase letter occurs.
        CaseSensitivity::Smart => {
            if pattern_lower == pattern {
                (pattern_lower.as_str(), CaseSensitivity::Insensitive)
            } else {
                (pattern, CaseSensitivity::Sensitive)
            }
        }
    };

    let glob = GlobBuilder::new(pattern)
        .case_insensitive(matches!(
            actual_case_sensitivity,
            CaseSensitivity::Insensitive
        ))
        .build()?
        .compile_matcher();

    let index = search_glob_fwd(context.tab_context_ref().curr_tab_ref(), &glob);
    if let Some(index) = index {
        cursor_move::cursor_move(context, index);
    }
    context.set_search_context(SearchContext::Glob(glob));
    Ok(())
}
