use crate::error::AppResult;
use crate::history::{create_dirlist_with_history, DirectoryHistory};
use crate::types::state::AppState;

use uuid::Uuid;

/// Re-reads only the parent/current/child listings of tab `id` that are marked stale or have
/// changed on disk.
pub fn soft_reload(app_state: &mut AppState, id: &Uuid) -> std::io::Result<()> {
    let mut dirlists = Vec::with_capacity(3);
    if let Some(curr_tab) = app_state.state.tab_state_ref().tab_ref(id) {
        let display_options = &app_state.config.display_options;
        let tab_options = app_state.state.tab_state_ref().curr_tab_ref().option_ref();
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

    if let Some(history) = app_state
        .state
        .tab_state_mut()
        .tab_mut(id)
        .map(|t| t.history_mut())
    {
        history.insert_entries(dirlists);
    }
    Ok(())
}

/// Soft-reloads the currently active tab. See [`soft_reload`].
pub fn soft_reload_curr_tab(app_state: &mut AppState) -> std::io::Result<()> {
    let curr_tab_id = app_state.state.tab_state_ref().curr_tab_id();
    soft_reload(app_state, &curr_tab_id)
}

/// Unconditionally re-reads the parent/current/child listings of tab `id` from disk.
pub fn reload(app_state: &mut AppState, id: &Uuid) -> std::io::Result<()> {
    let mut dirlists = Vec::with_capacity(3);
    if let Some(curr_tab) = app_state.state.tab_state_ref().tab_ref(id) {
        let display_options = &app_state.config.display_options;
        let tab_options = app_state.state.tab_state_ref().curr_tab_ref().option_ref();
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

    if let Some(history) = app_state
        .state
        .tab_state_mut()
        .tab_mut(id)
        .map(|t| t.history_mut())
    {
        history.insert_entries(dirlists);
    }
    app_state
        .state
        .message_queue_mut()
        .push_success("Directory listing reloaded!".to_string());
    Ok(())
}

/// Implements `reload_dirlist`: force-reloads the current tab and reports success.
pub fn reload_dirlist(app_state: &mut AppState) -> AppResult {
    reload(app_state, &app_state.state.tab_state_ref().curr_tab_id())?;
    Ok(())
}
