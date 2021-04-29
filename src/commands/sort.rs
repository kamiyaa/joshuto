use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;

use crate::util::load_child::LoadChild;
use crate::util::sort::SortType;

use super::reload;

pub fn set_sort(context: &mut JoshutoContext, method: SortType) -> JoshutoResult<()> {
    context.sort_options_mut().sort_method = method;

    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut().depreciate_all_entries();
    }

    reload::soft_reload(context.tab_context_ref().get_index(), context)?;
    LoadChild::load_child(context)?;
    Ok(())
}

pub fn toggle_reverse(context: &mut JoshutoContext) -> JoshutoResult<()> {
    context.sort_options_mut().reverse = !context.sort_options_ref().reverse;
    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut().depreciate_all_entries();
    }
    reload::soft_reload(context.tab_context_ref().get_index(), context)?;
    LoadChild::load_child(context)?;
    Ok(())
}
