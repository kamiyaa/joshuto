use crate::config::clean::app::display::sort_type::SortType;
use crate::context::AppContext;
use crate::error::AppResult;
use crate::history::DirectoryHistory;

use super::reload;

pub fn set_sort(context: &mut AppContext, method: SortType) -> AppResult {
    let curr_tab = context.tab_context_mut().curr_tab_mut();
    curr_tab
        .option_mut()
        .sort_options_mut()
        .set_sort_method(method);
    curr_tab.history_mut().depreciate_all_entries();
    refresh(context)
}

pub fn toggle_reverse(context: &mut AppContext) -> AppResult {
    let curr_tab = context.tab_context_mut().curr_tab_mut();
    let reversed = !curr_tab.option_mut().sort_options_ref().reverse;
    curr_tab.option_mut().sort_options_mut().reverse = reversed;
    curr_tab.history_mut().depreciate_all_entries();
    refresh(context)
}

fn refresh(context: &mut AppContext) -> AppResult {
    reload::soft_reload_curr_tab(context)?;
    Ok(())
}
