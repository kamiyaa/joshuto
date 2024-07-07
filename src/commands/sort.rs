use crate::error::AppResult;
use crate::history::DirectoryHistory;
use crate::types::option::sort::SortMethod;
use crate::types::state::AppState;

use super::reload;

pub fn set_sort(app_state: &mut AppState, method: SortMethod, reverse: Option<bool>) -> AppResult {
    let curr_tab = app_state.state.tab_state_mut().curr_tab_mut();
    curr_tab
        .option_mut()
        .sort_options_mut()
        .set_sort_method(method);
    curr_tab.history_mut().depreciate_all_entries();

    if let Some(r) = reverse {
        curr_tab.option_mut().sort_options_mut().reverse = r;
    }

    refresh(app_state)
}

pub fn toggle_reverse(app_state: &mut AppState) -> AppResult {
    let curr_tab = app_state.state.tab_state_mut().curr_tab_mut();
    let reversed = !curr_tab.option_mut().sort_options_ref().reverse;
    curr_tab.option_mut().sort_options_mut().reverse = reversed;
    curr_tab.history_mut().depreciate_all_entries();
    refresh(app_state)
}

fn refresh(app_state: &mut AppState) -> AppResult {
    reload::soft_reload_curr_tab(app_state)?;

    let ui_state = app_state.state.ui_state_ref().clone();
    let display_options = &app_state.config.display_options;
    let curr_tab = app_state.state.tab_state_mut().curr_tab_mut();

    macro_rules! update_viewport {
        ($x_list_mut: ident) => {
            if let Some(list) = curr_tab.$x_list_mut() {
                list.update_viewport(&ui_state, display_options);
            }
        };
    }

    update_viewport!(curr_list_mut);
    update_viewport!(parent_list_mut);
    update_viewport!(child_list_mut);

    Ok(())
}
