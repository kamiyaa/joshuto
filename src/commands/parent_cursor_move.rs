use std::path::PathBuf;

use crate::context::AppContext;
use crate::error::JoshutoResult;

pub fn parent_cursor_move(context: &mut AppContext, new_index: usize) -> JoshutoResult<()> {
    let mut path: Option<PathBuf> = None;
    let mut new_index = new_index;

    {
        let curr_tab = context.tab_context_mut().curr_tab_mut();
        if let Some(curr_list) = curr_tab.parent_list_mut() {
            if curr_list.index.is_some() {
                let dir_len = curr_list.contents.len();
                if new_index >= dir_len {
                    new_index = dir_len - 1;
                }
                let entry = &curr_list.contents[new_index];
                if entry.file_path().is_dir() {
                    curr_list.index = Some(new_index);
                    path = Some(entry.file_path().to_path_buf())
                }
            }
        }
        if let Some(path) = path.as_ref() {
            curr_tab.set_cwd(path);
        }
    }
    Ok(())
}

pub fn parent_up(context: &mut AppContext, u: usize) -> JoshutoResult<()> {
    let movement = match context.tab_context_ref().curr_tab_ref().parent_list_ref() {
        Some(list) => list.index.map(|idx| if idx > u { idx - u } else { 0 }),
        None => None,
    };

    if let Some(s) = movement {
        parent_cursor_move(context, s)?;
    }
    context.update_watcher();
    Ok(())
}

pub fn parent_down(context: &mut AppContext, u: usize) -> JoshutoResult<()> {
    let movement = match context.tab_context_ref().curr_tab_ref().parent_list_ref() {
        Some(list) => list.index.map(|idx| idx + u),
        None => None,
    };
    if let Some(s) = movement {
        parent_cursor_move(context, s)?;
    }
    Ok(())
}
