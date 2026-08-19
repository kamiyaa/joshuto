use std::path;

use crate::error::{AppError, AppErrorKind, AppResult};
use crate::fs::FileType;
use crate::types::state::AppState;
use crate::ui::AppBackend;

/// Updates the current entry's cached directory size from the listing cache, if it has since
/// been computed or changed there. Called before every cursor move.
pub fn lazy_load_directory_size(app_state: &mut AppState) {
    let directory_size = match app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|l| l.curr_entry_ref())
    {
        Some(curr_entry) => {
            if let Some(size) = curr_entry.metadata.directory_size() {
                let history = app_state.state.tab_state_ref().curr_tab_ref().history_ref();
                match history.get(curr_entry.file_path()).map(|d| d.len()) {
                    Some(len) if size != len => Some(len),
                    _ => None,
                }
            } else if curr_entry.metadata.file_type() != FileType::Directory {
                None
            } else {
                let history = app_state.state.tab_state_ref().curr_tab_ref().history_ref();
                history.get(curr_entry.file_path()).map(|d| d.len())
            }
        }
        None => None,
    };

    if let Some(s) = directory_size {
        if let Some(curr_entry) = app_state
            .state
            .tab_state_mut()
            .curr_tab_mut()
            .curr_list_mut()
            .and_then(|l| l.curr_entry_mut())
        {
            curr_entry.metadata.update_directory_size(s);
        }
    }
}

/// Moves the cursor in the current directory listing to `new_index`, clamped to the last entry.
/// The shared primitive underlying every cursor-movement command.
pub fn cursor_move(app_state: &mut AppState, new_index: usize) {
    lazy_load_directory_size(app_state);
    let mut new_index = new_index;
    let ui_state = app_state.state.ui_state_ref().clone();
    let display_options = &app_state.config.display_options;
    if let Some(curr_list) = app_state
        .state
        .tab_state_mut()
        .curr_tab_mut()
        .curr_list_mut()
    {
        if !curr_list.is_empty() {
            let dir_len = curr_list.len();
            if new_index >= dir_len {
                new_index = dir_len - 1;
            }
            curr_list.set_index(Some(new_index), &ui_state, display_options);
        }
    }
}

/// Moves the cursor onto the entry named by the first component of `path`, if present in the
/// current directory listing.
pub fn to_path(app_state: &mut AppState, path: &path::Path) -> AppResult {
    // This error should never happen
    let err = || AppError::new(AppErrorKind::UnknownError, String::from("Unexpected error"));
    let ui_state = app_state.state.ui_state_ref().clone();
    let display_options = &app_state.config.display_options;
    if let Some(curr_list) = app_state
        .state
        .tab_state_mut()
        .curr_tab_mut()
        .curr_list_mut()
    {
        if let path::Component::Normal(name) = path.components().next().ok_or_else(err)? {
            let index = curr_list.get_index_from_name(name.to_str().ok_or_else(err)?);
            curr_list.set_index(index, &ui_state, display_options);
        }
    }

    Ok(())
}

/// Implements `cursor_move_up`: moves the cursor up by `u`.
pub fn up(app_state: &mut AppState, u: usize) -> AppResult {
    let movement = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|list| list.get_index().map(|idx| idx.saturating_sub(u)));

    if let Some(s) = movement {
        cursor_move(app_state, s);
    }
    Ok(())
}

/// Implements `cursor_move_down`: moves the cursor down by `u`.
pub fn down(app_state: &mut AppState, u: usize) -> AppResult {
    let movement = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|list| list.get_index().map(|idx| idx.saturating_add(u)));

    if let Some(s) = movement {
        cursor_move(app_state, s);
    }
    Ok(())
}

/// Implements `cursor_move_home`: moves the cursor to the first entry.
pub fn home(app_state: &mut AppState) -> AppResult {
    let movement = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|curr_list| {
            let len = curr_list.len();
            if len == 0 {
                None
            } else {
                Some(0)
            }
        });

    if let Some(s) = movement {
        cursor_move(app_state, s);
    }
    Ok(())
}

/// Implements `cursor_move_end`: moves the cursor to the last entry.
pub fn end(app_state: &mut AppState) -> AppResult {
    let movement = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|curr_list| {
            let len = curr_list.len();
            if len == 0 {
                None
            } else {
                Some(len - 1)
            }
        });

    if let Some(s) = movement {
        cursor_move(app_state, s);
    }
    Ok(())
}

/// Returns how many entries make up one page, based on the terminal size and border settings.
fn get_page_size(app_state: &AppState, backend: &AppBackend) -> Option<usize> {
    let rect = backend.terminal.as_ref().map(|t| t.size())?.ok()?;

    let rect_height = rect.height as usize;
    if app_state.config.display_options.show_borders {
        if rect_height >= 4 {
            Some(rect_height - 4)
        } else {
            None
        }
    } else if rect_height >= 2 {
        Some(rect_height - 2)
    } else {
        None
    }
}

/// Implements `cursor_move_page_up`: moves the cursor up by `proportion` of a page.
pub fn page_up(app_state: &mut AppState, backend: &mut AppBackend, proportion: f64) -> AppResult {
    let page_size = get_page_size(app_state, backend).unwrap_or(10) as f64 * proportion;
    let page_size = page_size as usize;

    let movement = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|list| list.get_index().map(|idx| idx.saturating_sub(page_size)));

    if let Some(s) = movement {
        cursor_move(app_state, s);
    }
    Ok(())
}

/// Implements `cursor_move_page_down`: moves the cursor down by `proportion` of a page.
pub fn page_down(app_state: &mut AppState, backend: &mut AppBackend, proportion: f64) -> AppResult {
    let page_size = get_page_size(app_state, backend).unwrap_or(10) as f64 * proportion;
    let page_size = page_size as usize;

    let new_index = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|list| list.get_index().map(|idx| idx.saturating_add(page_size)));

    if let Some(idx) = new_index {
        cursor_move(app_state, idx);
    }
    Ok(())
}

/// Implements `cursor_move_page_home`: moves the cursor to the top of the current viewport.
pub fn page_home(app_state: &mut AppState, _: &mut AppBackend) -> AppResult {
    let new_index = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|curr_list| curr_list.first_index_for_viewport());
    if let Some(idx) = new_index {
        cursor_move(app_state, idx);
    }
    Ok(())
}

/// Implements `cursor_move_page_middle`: moves the cursor to the middle of the current viewport.
pub fn page_middle(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    let movement = get_page_size(app_state, backend).unwrap_or(10) / 2;

    let new_index = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|curr_list| curr_list.first_index_for_viewport() + movement);
    if let Some(idx) = new_index {
        cursor_move(app_state, idx);
    }
    Ok(())
}

/// Implements `cursor_move_page_end`: moves the cursor to the bottom of the current viewport.
pub fn page_end(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    let movement = get_page_size(app_state, backend).unwrap_or(10) - 1;

    let new_index = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|curr_list| curr_list.first_index_for_viewport() + movement);
    if let Some(idx) = new_index {
        cursor_move(app_state, idx);
    }
    Ok(())
}
