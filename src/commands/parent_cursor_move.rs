use std::path::PathBuf;

use crate::context::AppContext;
use crate::error::AppResult;

pub fn parent_cursor_move(context: &mut AppContext, new_index: usize) -> AppResult {
    let mut path: Option<PathBuf> = None;
    let mut new_index = new_index;

    {
        let ui_context = context.ui_context_ref().clone();
        let display_options = context.config_ref().display_options_ref().clone();
        let curr_tab = context.tab_context_mut().curr_tab_mut();
        if let Some(curr_list) = curr_tab.parent_list_mut() {
            if curr_list.get_index().is_some() {
                let dir_len = curr_list.contents.len();
                if new_index >= dir_len {
                    new_index = dir_len - 1;
                }
                let entry = &curr_list.contents[new_index];
                if entry.file_path().is_dir() {
                    path = Some(entry.file_path().to_path_buf());
                    curr_list.set_index(Some(new_index), &ui_context, &display_options);
                }
            }
        }
        if let Some(path) = path.as_ref() {
            curr_tab.set_cwd(path);
        }
    }
    Ok(())
}

pub fn parent_up(context: &mut AppContext, u: usize) -> AppResult {
    let movement = context
        .tab_context_ref()
        .curr_tab_ref()
        .parent_list_ref()
        .and_then(|list| list.get_index().map(|idx| idx.saturating_sub(u)));

    if let Some(s) = movement {
        parent_cursor_move(context, s)?;
    }
    context.update_watcher();
    Ok(())
}

pub fn parent_down(context: &mut AppContext, u: usize) -> AppResult {
    let movement = context
        .tab_context_ref()
        .curr_tab_ref()
        .parent_list_ref()
        .and_then(|list| list.get_index().map(|idx| idx + u));

    if let Some(s) = movement {
        parent_cursor_move(context, s)?;
    }
    Ok(())
}
