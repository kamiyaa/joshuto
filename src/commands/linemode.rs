use super::reload;
use crate::error::AppResult;
use crate::history::DirectoryHistory;
use crate::types::option::line_mode::LineMode;
use crate::types::state::AppState;

pub fn set_linemode(app_state: &mut AppState, linemode: LineMode) -> AppResult {
    let curr_tab = app_state.state.tab_state_mut().curr_tab_mut();
    curr_tab.option_mut().linemode = linemode;
    curr_tab.history_mut().depreciate_all_entries();
    reload::soft_reload_curr_tab(app_state)?;
    Ok(())
}
