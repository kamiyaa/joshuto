use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::ui::TuiBackend;
use std::path::PathBuf;

pub fn cursor_move(new_index: usize, context: &mut JoshutoContext) -> JoshutoResult<()> {
    let mut path: Option<PathBuf> = None;
    let mut new_index = new_index;

    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        if curr_list.index.is_some() {
            let dir_len = curr_list.contents.len();
            if new_index >= dir_len {
                new_index = dir_len - 1;
            }
            curr_list.index = Some(new_index);

            let entry = &curr_list.contents[new_index];
            path = Some(entry.file_path().to_path_buf())
        }
    }

    // get preview
    if let Some(path) = path {
        if path.is_dir() {
            let sort_options = context.config_t.sort_option.clone();
            context
                .tab_context_mut()
                .curr_tab_mut()
                .history_mut()
                .create_or_soft_update(path.as_path(), &sort_options)?;
        }
    }
    Ok(())
}

pub fn up(context: &mut JoshutoContext, u: usize) -> JoshutoResult<()> {
    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => curr_list.index.map(|idx| if idx > u { idx - u } else { 0 }),
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn down(context: &mut JoshutoContext, u: usize) -> JoshutoResult<()> {
    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => curr_list.index.map(|idx| idx + u),
        None => None,
    };
    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn home(context: &mut JoshutoContext) -> JoshutoResult<()> {
    let movement: Option<usize> = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            let len = curr_list.contents.len();
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

pub fn end(context: &mut JoshutoContext) -> JoshutoResult<()> {
    let movement: Option<usize> = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            let len = curr_list.contents.len();
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

pub fn page_up(context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let half_page = {
        match backend.terminal.as_ref().unwrap().size() {
            Ok(rect) => rect.height as usize - 2,
            _ => 10,
        }
    };

    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => curr_list
            .index
            .map(|idx| if idx > half_page { idx - half_page } else { 0 }),
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn page_down(context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let half_page = {
        match backend.terminal.as_ref().unwrap().size() {
            Ok(rect) => rect.height as usize - 2,
            _ => 10,
        }
    };

    let movement = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            let dir_len = curr_list.contents.len();
            curr_list.index.map(|idx| {
                if idx + half_page > dir_len - 1 {
                    dir_len - 1
                } else {
                    idx + half_page
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
