use crate::config::clean::app::tab::TabBarDisplayMode;
use crate::context::AppContext;
use crate::error::AppResult;

pub fn set_tab_bar_display_mode(
    context: &mut AppContext,
    mode: &TabBarDisplayMode,
) -> AppResult<()> {
    context.tab_context_mut().display.mode = *mode;
    Ok(())
}
