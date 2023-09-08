use crate::context::AppContext;
use crate::error::AppResult;
use crate::history::DirectoryHistory;

use super::reload;

pub fn _toggle_hidden(context: &mut AppContext) {
    let opposite = !context.config_ref().display_options_ref().show_hidden();
    context
        .config_mut()
        .display_options_mut()
        .set_show_hidden(opposite);

    for (_, tab) in context.tab_context_mut().iter_mut() {
        tab.history_mut().depreciate_all_entries();
        if let Some(s) = tab.curr_list_mut() {
            s.depreciate();
        }
    }
}

pub fn toggle_hidden(context: &mut AppContext) -> AppResult {
    _toggle_hidden(context);
    reload::soft_reload_curr_tab(context)?;
    Ok(())
}
