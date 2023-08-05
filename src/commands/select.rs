use globset::GlobBuilder;

use crate::config::option::{CaseSensitivity, SelectOption};
use crate::context::AppContext;
use crate::error::JoshutoResult;

use super::cursor_move;

pub fn select_files(
    context: &mut AppContext,
    pattern: &str,
    options: &SelectOption,
) -> JoshutoResult {
    if pattern.is_empty() {
        select_without_pattern(context, options)
    } else {
        select_with_pattern(context, pattern, options)
    }
}

fn select_without_pattern(context: &mut AppContext, options: &SelectOption) -> JoshutoResult {
    if options.all {
        if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
            curr_list.iter_mut().for_each(|e| {
                if options.reverse {
                    e.set_permanent_selected(false);
                } else if options.toggle {
                    e.set_permanent_selected(!e.is_selected());
                } else {
                    e.set_permanent_selected(true);
                }
            });
        }
    } else if let Some(entry) = context
        .tab_context_mut()
        .curr_tab_mut()
        .curr_list_mut()
        .and_then(|s| s.curr_entry_mut())
    {
        if options.reverse {
            entry.set_permanent_selected(false);
        } else if options.toggle {
            entry.set_permanent_selected(!entry.is_selected());
        } else {
            entry.set_permanent_selected(true);
        }
        cursor_move::down(context, 1)?;
    }
    Ok(())
}

fn select_with_pattern(
    context: &mut AppContext,
    pattern: &str,
    options: &SelectOption,
) -> JoshutoResult {
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

    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        let mut found = 0;
        curr_list
            .iter_mut()
            .filter(|e| glob.is_match(e.file_name()))
            .for_each(|e| {
                found += 1;
                if options.reverse {
                    e.set_permanent_selected(false);
                } else if options.toggle {
                    e.set_permanent_selected(!e.is_selected());
                } else {
                    e.set_permanent_selected(true);
                }
            });
        context
            .message_queue_mut()
            .push_info(format!("{} files selected", found));
    }
    Ok(())
}
