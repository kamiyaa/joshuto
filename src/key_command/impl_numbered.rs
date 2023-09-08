use crate::commands::*;
use crate::config::clean::keymap::AppKeyMapping;
use crate::context::AppContext;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::ui::AppBackend;

use super::{Command, NumberedExecute};

// In joshuto you can prefix simple commands with numbers by entering number,
// and then pressing key which some command is bound to. This is used mainly
// for easier navigation. You don't have to implement this for every command
impl NumberedExecute for Command {
    #[allow(unused)] // backend and keymap_t args are not used, but they probably will be
    fn numbered_execute(
        &self,
        number_prefix: usize,
        context: &mut AppContext,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult {
        match self {
            Self::CursorMoveUp { .. } => cursor_move::up(context, number_prefix),
            Self::CursorMoveDown { .. } => cursor_move::down(context, number_prefix),
            _ => Err(AppError::new(
                AppErrorKind::UnrecognizedCommand,
                "Command cannot be prefixed by a number".to_string(),
            )),
        }
    }
}
