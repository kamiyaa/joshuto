use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::util::sort_type::SortType;

use super::reload;

pub fn set_sort(context: &mut AppContext, method: SortType) -> JoshutoResult<()> {
    context
        .config_mut()
        .sort_options_mut()
        .set_sort_method(method);
    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut().depreciate_all_entries();
    }
    refresh(context)
}

pub fn toggle_reverse(context: &mut AppContext) -> JoshutoResult<()> {
    let reversed = !context.config_ref().sort_options_ref().reverse;
    context.config_mut().sort_options_mut().reverse = reversed;

    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut().depreciate_all_entries();
    }
    refresh(context)
}

fn refresh(context: &mut AppContext) -> JoshutoResult<()> {
    reload::soft_reload(context.tab_context_ref().index, context)?;
    Ok(())
}
