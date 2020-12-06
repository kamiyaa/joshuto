use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;

use super::reload;

pub fn _toggle_hidden(context: &mut JoshutoContext) {
    let opposite = !context.config_t.sort_option.show_hidden;
    context.config_t.sort_option.show_hidden = opposite;

    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut().depreciate_all_entries();
        if let Some(s) = tab.curr_list_mut() {
            s.depreciate();
        }
    }
}

pub fn toggle_hidden(context: &mut JoshutoContext) -> JoshutoResult<()> {
    _toggle_hidden(context);
    reload::reload_dirlist(context)
}
