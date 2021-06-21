use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

use std::path::PathBuf;

pub fn cursor_move(new_index: usize, context: &mut AppContext) -> JoshutoResult<()> {
    let mut path: Option<PathBuf> = None;
    let mut new_index = new_index;

    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        if !curr_list.is_empty() {
            let dir_len = curr_list.len();
            if new_index >= dir_len {
                new_index = dir_len - 1;
            }
            curr_list.index = Some(new_index);

            let entry = &curr_list.contents[new_index];
            path = Some(entry.file_path().to_path_buf())
        }
    }
    Ok(())
}

pub fn up(context: &mut AppContext, u: usize) -> JoshutoResult<()> {
    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => curr_list.index.map(|idx| if idx > u { idx - u } else { 0 }),
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn down(context: &mut AppContext, u: usize) -> JoshutoResult<()> {
    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => curr_list.index.map(|idx| idx + u),
        None => None,
    };
    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn home(context: &mut AppContext) -> JoshutoResult<()> {
    let movement: Option<usize> = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            let len = curr_list.len();
            if len == 0 {
                None
            } else {
                Some(0)
            }
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn end(context: &mut AppContext) -> JoshutoResult<()> {
    let movement: Option<usize> = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            let len = curr_list.len();
            if len == 0 {
                None
            } else {
                Some(len - 1)
            }
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

fn get_page_size(context: &AppContext, backend: &TuiBackend) -> Option<usize> {
    let config = context.config_ref();
    let rect = backend.terminal.as_ref().map(|t| t.size())?.ok()?;

    let rect_height = rect.height as usize;
    if config.display_options_ref().show_borders() {
        if rect_height >= 4 {
            Some(rect_height - 4)
        } else {
            None
        }
    } else {
        if rect_height >= 2 {
            Some(rect_height - 2)
        } else {
            None
        }
    }
}

pub fn page_up(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let page_size = get_page_size(context, backend).unwrap_or(10);

    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => curr_list
            .index
            .map(|idx| if idx > page_size { idx - page_size } else { 0 }),
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn page_down(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let page_size = get_page_size(context, backend).unwrap_or(10);

    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            let dir_len = curr_list.len();
            curr_list.index.map(|idx| {
                if idx + page_size > dir_len - 1 {
                    dir_len - 1
                } else {
                    idx + page_size
                }
            })
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}
