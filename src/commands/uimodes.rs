use crate::error::AppResult;
use crate::types::state::AppState;

/// Implements `toggle_visual_mode`: enters or exits visual (range) selection mode.
pub fn toggle_visual_mode(app_state: &mut AppState) -> AppResult {
    if let Some(curr_dir_list) = app_state
        .state
        .tab_state_mut()
        .curr_tab_mut()
        .curr_list_mut()
    {
        curr_dir_list.toggle_visual_mode()
    };
    Ok(())
}
