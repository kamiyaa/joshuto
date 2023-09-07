use std::path::PathBuf;

use crate::context::AppContext;
use crate::error::AppResult;
use crate::preview::preview_file::PreviewFileState;

fn preview_cursor_move(context: &mut AppContext, new_index: usize) -> AppResult {
    let file_path: Option<PathBuf> = {
        let curr_tab = context.tab_context_ref().curr_tab_ref();
        let curr_list = curr_tab.curr_list_ref();
        let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());
        curr_entry.map(|e| e.file_path().to_path_buf())
    };

    let preview_context = context.preview_context_mut();
    if let Some(file_path) = file_path {
        if let Some(PreviewFileState::Success { data }) =
            preview_context.previews_mut().get_mut(&file_path)
        {
            data.index = new_index;
        }
    }
    Ok(())
}

pub fn preview_up(context: &mut AppContext, u: usize) -> AppResult {
    let new_index = {
        let curr_tab = context.tab_context_ref().curr_tab_ref();
        let curr_list = curr_tab.curr_list_ref();
        let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());
        let file_path = curr_entry.map(|e| e.file_path());

        let preview_context = context.preview_context_ref();
        if let Some(file_path) = file_path {
            if let Some(PreviewFileState::Success { data }) =
                preview_context.previews_ref().get(file_path)
            {
                if data.index < u {
                    Some(0)
                } else {
                    Some(data.index - u)
                }
            } else {
                None
            }
        } else {
            None
        }
    };
    if let Some(new_index) = new_index {
        preview_cursor_move(context, new_index)?;
    }
    Ok(())
}

pub fn preview_down(context: &mut AppContext, u: usize) -> AppResult {
    let new_index = {
        let curr_tab = context.tab_context_ref().curr_tab_ref();
        let curr_list = curr_tab.curr_list_ref();
        let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());
        let file_path = curr_entry.map(|e| e.file_path());

        let preview_context = context.preview_context_ref();
        if let Some(file_path) = file_path {
            if let Some(PreviewFileState::Success { data }) =
                preview_context.previews_ref().get(file_path)
            {
                Some(data.index + u)
            } else {
                None
            }
        } else {
            None
        }
    };
    if let Some(new_index) = new_index {
        preview_cursor_move(context, new_index)?;
    }
    Ok(())
}
