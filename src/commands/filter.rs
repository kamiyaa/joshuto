use crate::context::AppContext;
use crate::error::JoshutoResult;

use super::reload;

pub fn filter(context: &mut AppContext, arg: &str) -> JoshutoResult {
    let curr_tab = context.tab_context_mut().curr_tab_mut();
    curr_tab
        .option_mut()
        .set_filter_string(arg);

    if let Some(list) = curr_tab.curr_list_mut() {
        list.depreciate();
    }

    reload::soft_reload_curr_tab(context)?;
    Ok(())
}
