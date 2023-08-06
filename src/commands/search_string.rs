use crate::config::option::CaseSensitivity;
use crate::context::AppContext;
use crate::tab::JoshutoTab;
use crate::util::search::SearchContext;

use super::cursor_move;

pub fn search_string_fwd(
    curr_tab: &JoshutoTab,
    pattern: &str,
    case_sensitivity: CaseSensitivity,
) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let offset = curr_list.get_index()? + 1;
    let contents_len = curr_list.contents.len();
    for i in 0..contents_len {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();
        let file_name = match case_sensitivity {
            CaseSensitivity::Insensitive => file_name.to_lowercase(),
            CaseSensitivity::Sensitive => file_name.to_string(),
            CaseSensitivity::Smart => unreachable!(),
        };
        if file_name.contains(pattern) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}

pub fn search_string_start(
    curr_tab: &JoshutoTab,
    pattern: &str,
    case_sensitivity: CaseSensitivity,
) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let contents_len = curr_list.contents.len();
    for i in 0..contents_len {
        let file_name = curr_list.contents[i].file_name();
        let file_name = match case_sensitivity {
            CaseSensitivity::Insensitive => file_name.to_lowercase(),
            CaseSensitivity::Sensitive => file_name.to_string(),
            CaseSensitivity::Smart => unreachable!(),
        };
        if file_name.contains(pattern) {
            return Some(i);
        }
    }
    None
}

pub fn search_string_rev(
    curr_tab: &JoshutoTab,
    pattern: &str,
    case_sensitivity: CaseSensitivity,
) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let offset = curr_list.get_index()?;
    let contents_len = curr_list.contents.len();
    for i in (0..contents_len).rev() {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();
        let file_name = match case_sensitivity {
            CaseSensitivity::Insensitive => file_name.to_lowercase(),
            CaseSensitivity::Sensitive => file_name.to_string(),
            CaseSensitivity::Smart => unreachable!(),
        };
        if file_name.contains(pattern) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}

pub fn search_string(context: &mut AppContext, pattern: &str, incremental: bool) {
    let curr_tab = context.tab_context_ref().curr_tab_ref();

    let case_sensitivity = context.config_ref().search_options_ref().case_sensitivity;
    let search_context = SearchContext::new_string(pattern, case_sensitivity);

    let (pattern, actual_case_sensitivity) = match &search_context {
        SearchContext::String {
            pattern,
            actual_case_sensitivity,
        } => (pattern, *actual_case_sensitivity),
        _ => unreachable!(),
    };

    let index = if incremental {
        search_string_start(curr_tab, pattern, actual_case_sensitivity)
    } else {
        search_string_fwd(curr_tab, pattern, actual_case_sensitivity)
    };

    if let Some(index) = index {
        cursor_move::cursor_move(context, index);
    }

    context.set_search_context(search_context);
}
