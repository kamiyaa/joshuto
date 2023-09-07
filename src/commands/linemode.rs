use super::reload;
use crate::config::clean::app::display::line_mode::LineMode;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;

pub fn set_linemode(context: &mut AppContext, linemode: LineMode) -> JoshutoResult {
    let curr_tab = context.tab_context_mut().curr_tab_mut();
    curr_tab.option_mut().linemode = linemode;
    curr_tab.history_mut().depreciate_all_entries();
    reload::soft_reload_curr_tab(context)?;
    Ok(())
}
