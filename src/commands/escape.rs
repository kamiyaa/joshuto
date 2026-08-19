use crate::error::AppResult;
use crate::types::state::AppState;

/// Implements `escape`: cancels the current visual-mode selection, if any.
pub fn escape(app_state: &mut AppState) -> AppResult {
    if let Some(curr_dir_list) = app_state
        .state
        .tab_state_mut()
        .curr_tab_mut()
        .curr_list_mut()
    {
        curr_dir_list.visual_mode_cancel();
    };
    Ok(())
}
