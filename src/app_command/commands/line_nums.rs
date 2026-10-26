use crate::config::LineNumberStyle;
use crate::error::AppResult;
use crate::state::AppState;

use super::reload;

/// Implements `line_nums`: changes the line-number display style and reloads the current listing.
pub fn switch_line_numbering(app_state: &mut AppState, style: LineNumberStyle) -> AppResult {
    app_state.config.display_options.line_number_style = style;
    reload::reload_dirlist(app_state)
}
