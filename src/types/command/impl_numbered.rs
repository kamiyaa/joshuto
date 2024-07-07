use crate::commands::*;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::traits::app_execute::NumberedExecute;
use crate::types::keymap::AppKeyMapping;
use crate::types::state::AppState;
use crate::ui::AppBackend;

use super::Command;

// In joshuto you can prefix simple commands with numbers by entering number,
// and then pressing key which some command is bound to. This is used mainly
// for easier navigation. You don't have to implement this for every command
impl NumberedExecute for Command {
    #[allow(unused)] // backend and keymap_t args are not used, but they probably will be
    fn numbered_execute(
        &self,
        number_prefix: usize,
        app_state: &mut AppState,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult {
        match self {
            Self::CursorMoveUp { .. } => cursor_move::up(app_state, number_prefix),
            Self::CursorMoveDown { .. } => cursor_move::down(app_state, number_prefix),
            _ => Err(AppError::new(
                AppErrorKind::UnrecognizedCommand,
                "Command cannot be prefixed by a number".to_string(),
            )),
        }
    }
}
