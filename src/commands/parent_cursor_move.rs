use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use std::path::PathBuf;

pub fn parent_cursor_move(new_index: usize, context: &mut JoshutoContext) -> JoshutoResult<()> {
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
            curr_tab.set_pwd(path);
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

pub fn parent_up(context: &mut JoshutoContext, u: usize) -> JoshutoResult<()> {
    let movement = match context.tab_context_ref().curr_tab_ref().parent_list_ref() {
        Some(list) => list.index.map(|idx| if idx > u { idx - u } else { 0 }),
        None => None,
    };

    if let Some(s) = movement {
        parent_cursor_move(s, context)?;
    }
    Ok(())
}

pub fn parent_down(context: &mut JoshutoContext, u: usize) -> JoshutoResult<()> {
    let movement = match context.tab_context_ref().curr_tab_ref().parent_list_ref() {
        Some(list) => list.index.map(|idx| idx + u),
        None => None,
    };
    if let Some(s) = movement {
        parent_cursor_move(s, context)?;
    }
    Ok(())
}
