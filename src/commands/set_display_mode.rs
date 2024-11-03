use crate::error::AppResult;
use crate::types::option::display::DisplayMode;
use crate::types::state::AppState;

use super::reload;

pub fn set_display_mode(app_state: &mut AppState, mode: DisplayMode) -> AppResult {
    app_state.config.display_options.mode = mode;
    reload::soft_reload_curr_tab(app_state)?;
    Ok(())
}
