use std::io;

use crate::context::AppContext;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::ui::AppBackend;

use super::fzf;
use super::select::SelectOption;

pub fn select_fzf(
    context: &mut AppContext,
    backend: &mut AppBackend,
    options: &SelectOption,
) -> AppResult {
    let items = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|list| {
            let v: Vec<String> = list
                .iter()
                .enumerate()
                .map(|(i, entry)| format!("{} {}\n", i, entry.file_name()))
                .collect();
            v
        })
        .unwrap_or_default();

    if items.is_empty() {
        return Err(AppError::new(
            AppErrorKind::Io(io::ErrorKind::InvalidData),
            "no files to select".to_string(),
        ));
    }

    let fzf_output = fzf::fzf_multi(context, backend, items)?;

    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        let mut found = 0;

        for selected in fzf_output.lines() {
            if let Some((selected_idx_str, _)) = selected.split_once(' ') {
                if let Ok(index) = selected_idx_str.parse::<usize>() {
                    let entry = curr_list.contents.get_mut(index).unwrap();
                    found += 1;

                    if options.reverse {
                        entry.set_permanent_selected(false);
                    } else if options.toggle {
                        entry.set_permanent_selected(!entry.is_selected());
                    } else {
                        entry.set_permanent_selected(true);
                    }
                }
            }
        }

        context
            .message_queue_mut()
            .push_info(format!("{} files selected", found));
    }

    Ok(())
}
