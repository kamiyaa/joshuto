use crate::error::AppResult;
use crate::types::state::{AppState, MatchState};

use super::reload;

pub fn filter(app_state: &mut AppState, filter_state: MatchState) -> AppResult {
    let curr_tab = app_state.state.tab_state_mut().curr_tab_mut();
    let path = curr_tab.get_cwd().to_path_buf();

    curr_tab
        .option_mut()
        .dirlist_options_mut(&path)
        .set_filter_state(filter_state);

    if let Some(list) = curr_tab.curr_list_mut() {
        list.depreciate();
    }

    reload::soft_reload_curr_tab(app_state)?;
    Ok(())
}
