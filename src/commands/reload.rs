use crate::context::AppContext;
use crate::error::AppResult;
use crate::history::create_dirlist_with_history;

use uuid::Uuid;

// reload only if we have a queued reload
pub fn soft_reload(context: &mut AppContext, id: &Uuid) -> std::io::Result<()> {
    let mut paths = Vec::with_capacity(3);
    if let Some(curr_tab) = context.tab_context_ref().tab_ref(id) {
        if let Some(curr_list) = curr_tab.curr_list_ref() {
            if curr_list.need_update() {
                paths.push(curr_list.file_path().to_path_buf());
            }
        }
        if let Some(curr_list) = curr_tab.parent_list_ref() {
            if curr_list.need_update() {
                paths.push(curr_list.file_path().to_path_buf());
            }
        }
        if let Some(curr_list) = curr_tab.child_list_ref() {
            if curr_list.need_update() {
                paths.push(curr_list.file_path().to_path_buf());
            }
        }
    }

    if !paths.is_empty() {
        let options = context.config_ref().display_options_ref().clone();
        let tab_options = context
            .tab_context_ref()
            .curr_tab_ref()
            .option_ref()
            .clone();
        if let Some(history) = context
            .tab_context_mut()
            .tab_mut(id)
            .map(|t| t.history_mut())
        {
            for path in paths {
                let new_dirlist =
                    create_dirlist_with_history(history, path.as_path(), &options, &tab_options)?;
                history.insert(path, new_dirlist);
            }
        }
    }
    Ok(())
}

pub fn soft_reload_curr_tab(context: &mut AppContext) -> std::io::Result<()> {
    let curr_tab_id = context.tab_context_ref().curr_tab_id();
    soft_reload(context, &curr_tab_id)
}

pub fn reload(context: &mut AppContext, id: &Uuid) -> std::io::Result<()> {
    let mut paths = Vec::with_capacity(3);
    if let Some(curr_tab) = context.tab_context_ref().tab_ref(id) {
        if let Some(curr_list) = curr_tab.curr_list_ref() {
            paths.push(curr_list.file_path().to_path_buf());
        }
        if let Some(curr_list) = curr_tab.parent_list_ref() {
            paths.push(curr_list.file_path().to_path_buf());
        }
        if let Some(curr_list) = curr_tab.child_list_ref() {
            paths.push(curr_list.file_path().to_path_buf());
        }
    }

    if !paths.is_empty() {
        let options = context.config_ref().display_options_ref().clone();
        let tab_options = context
            .tab_context_ref()
            .curr_tab_ref()
            .option_ref()
            .clone();
        if let Some(history) = context
            .tab_context_mut()
            .tab_mut(id)
            .map(|t| t.history_mut())
        {
            for path in paths {
                let new_dirlist =
                    create_dirlist_with_history(history, path.as_path(), &options, &tab_options)?;
                history.insert(path, new_dirlist);
            }
        }
    }
    context
        .message_queue_mut()
        .push_success("Directory listing reloaded!".to_string());
    Ok(())
}

pub fn reload_dirlist(context: &mut AppContext) -> AppResult {
    reload(context, &context.tab_context_ref().curr_tab_id())?;
    Ok(())
}
