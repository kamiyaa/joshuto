use crate::config::option::SortType;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;

use super::reload;

pub fn set_sort(context: &mut AppContext, method: SortType) -> JoshutoResult {
    let curr_tab = context.tab_context_mut().curr_tab_mut();
    curr_tab
        .option_mut()
        .sort_options_mut()
        .set_sort_method(method);
    curr_tab.history_mut().depreciate_all_entries();
    refresh(context)
}

pub fn toggle_reverse(context: &mut AppContext) -> JoshutoResult {
    let curr_tab = context.tab_context_mut().curr_tab_mut();
    let reversed = !curr_tab.option_mut().sort_options_ref().reverse;
    curr_tab.option_mut().sort_options_mut().reverse = reversed;
    curr_tab.history_mut().depreciate_all_entries();
    refresh(context)
}

fn refresh(context: &mut AppContext) -> JoshutoResult {
    reload::soft_reload(context.tab_context_ref().index, context)?;
    Ok(())
}
