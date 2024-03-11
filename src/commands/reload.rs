use crate::context::AppContext;
use crate::error::AppResult;
use crate::history::{create_dirlist_with_history, DirectoryHistory};

use uuid::Uuid;

// reload only if we have a queued reload
pub fn soft_reload(context: &mut AppContext, id: &Uuid) -> std::io::Result<()> {
    let mut dirlists = Vec::with_capacity(3);
    if let Some(curr_tab) = context.tab_context_ref().tab_ref(id) {
        let config = context.config_ref();
        let display_options = context.config_ref().display_options_ref();
        let tab_options = context.tab_context_ref().curr_tab_ref().option_ref();
        let history = curr_tab.history_ref();
        for curr_list in [
            curr_tab.parent_list_ref(),
            curr_tab.curr_list_ref(),
            curr_tab.child_list_ref(),
        ]
        .into_iter()
        .flatten()
        {
            if curr_list.need_update() {
                let new_dirlist = create_dirlist_with_history(
                    history,
                    curr_list.file_path(),
                    display_options,
                    tab_options,
                )?;
                dirlists.push(new_dirlist);
            }
        }
    }

    if let Some(history) = context
        .tab_context_mut()
        .tab_mut(id)
        .map(|t| t.history_mut())
    {
        history.insert_entries(dirlists);
    }
    Ok(())
}

pub fn soft_reload_curr_tab(context: &mut AppContext) -> std::io::Result<()> {
    let curr_tab_id = context.tab_context_ref().curr_tab_id();
    soft_reload(context, &curr_tab_id)
}

pub fn reload(context: &mut AppContext, id: &Uuid) -> std::io::Result<()> {
    let mut dirlists = Vec::with_capacity(3);
    if let Some(curr_tab) = context.tab_context_ref().tab_ref(id) {
        let config = context.config_ref();
        let display_options = context.config_ref().display_options_ref();
        let tab_options = context.tab_context_ref().curr_tab_ref().option_ref();
        let history = curr_tab.history_ref();
        for curr_list in [
            curr_tab.parent_list_ref(),
            curr_tab.curr_list_ref(),
            curr_tab.child_list_ref(),
        ]
        .into_iter()
        .flatten()
        {
            let new_dirlist = create_dirlist_with_history(
                history,
                curr_list.file_path(),
                display_options,
                tab_options,
            )?;
            dirlists.push(new_dirlist);
        }
    }

    if let Some(history) = context
        .tab_context_mut()
        .tab_mut(id)
        .map(|t| t.history_mut())
    {
        history.insert_entries(dirlists);
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
