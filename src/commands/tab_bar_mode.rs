use crate::{config::option::TabBarDisplayMode, context::AppContext, error::JoshutoError};

pub fn set_tab_bar_display_mode(
    context: &mut AppContext,
    mode: &TabBarDisplayMode,
) -> Result<(), JoshutoError> {
    context.tab_context_mut().display.mode = *mode;
    Ok(())
}
