use crate::error::AppResult;
use crate::fs::DirectoryHistory;
use crate::state::AppState;

use super::reload;

/// Flips the show-git-status setting and invalidates all cached listings, without reloading.
pub fn _toggle_git_status(app_state: &mut AppState) {
    let opposite = !app_state.config.display_options.show_git_status;
    app_state.config.display_options.show_git_status = opposite;

    for (_, tab) in app_state.state.tab_state.iter_mut() {
        tab.history.depreciate_all_entries();
        if let Some(s) = tab.curr_list_mut() {
            s.depreciate();
        }
    }
}

/// Implements `toggle_git_status`: toggles git status indicators and reloads the current tab.
pub fn toggle_git_status(app_state: &mut AppState) -> AppResult {
    _toggle_git_status(app_state);
    reload::soft_reload_curr_tab(app_state)?;
    Ok(())
}
