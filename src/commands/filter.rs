use crate::context::{AppContext, MatchContext};
use crate::error::AppResult;

use super::reload;

pub fn filter(context: &mut AppContext, filter_context: MatchContext) -> AppResult {
    let curr_tab = context.tab_context_mut().curr_tab_mut();
    let path = curr_tab.cwd().to_path_buf();

    curr_tab
        .option_mut()
        .dirlist_options_mut(&path)
        .set_filter_context(filter_context);

    if let Some(list) = curr_tab.curr_list_mut() {
        list.depreciate();
    }

    reload::soft_reload_curr_tab(context)?;
    Ok(())
}
