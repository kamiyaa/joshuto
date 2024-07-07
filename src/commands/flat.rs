use crate::error::AppResult;
use crate::types::state::AppState;

use super::reload;

pub fn flatten(app_state: &mut AppState, depth: usize) -> AppResult {
    let curr_tab = app_state.state.tab_state_mut().curr_tab_mut();
    let path = curr_tab.get_cwd().to_path_buf();
    curr_tab
        .option_mut()
        .dirlist_options_mut(&path)
        .set_depth(depth as u8);

    if let Some(list) = curr_tab.curr_list_mut() {
        list.depreciate();
    }

    reload::soft_reload_curr_tab(app_state)?;
    Ok(())
}
