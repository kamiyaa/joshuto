use std::path::PathBuf;

use crate::error::AppResult;
use crate::preview::preview_file::PreviewFileState;
use crate::types::state::AppState;

fn preview_cursor_move(app_state: &mut AppState, new_index: usize) -> AppResult {
    let file_path: Option<PathBuf> = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|c| c.curr_entry_ref())
        .map(|e| e.file_path().to_path_buf());

    let preview_state = app_state.state.preview_state_mut();
    if let Some(file_path) = file_path {
        if let Some(PreviewFileState::Success(data)) =
            preview_state.previews_mut().get_mut(&file_path)
        {
            data.index = new_index;
        }
    }
    Ok(())
}

pub fn preview_up(app_state: &mut AppState, u: usize) -> AppResult {
    let new_index = {
        let file_path = app_state
            .state
            .tab_state_ref()
            .curr_tab_ref()
            .curr_list_ref()
            .and_then(|c| c.curr_entry_ref())
            .map(|e| e.file_path());

        let preview_state = app_state.state.preview_state_ref();
        if let Some(file_path) = file_path {
            if let Some(PreviewFileState::Success(data)) =
                preview_state.previews_ref().get(file_path)
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
        preview_cursor_move(app_state, new_index)?;
    }
    Ok(())
}

pub fn preview_down(app_state: &mut AppState, u: usize) -> AppResult {
    let new_index = {
        let file_path = app_state
            .state
            .tab_state_ref()
            .curr_tab_ref()
            .curr_list_ref()
            .and_then(|c| c.curr_entry_ref())
            .map(|e| e.file_path());

        let preview_state = app_state.state.preview_state_ref();
        if let Some(file_path) = file_path {
            // TODO: scroll in child list
            if let Some(PreviewFileState::Success(data)) =
                preview_state.previews_ref().get(file_path)
            {
                if (data.index as isize)
                    < (data.output.split('\n').count() as isize - u as isize - 3)
                {
                    Some(data.index + u)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };
    if let Some(new_index) = new_index {
        preview_cursor_move(app_state, new_index)?;
    }
    Ok(())
}
