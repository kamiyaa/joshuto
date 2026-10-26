use crate::{
    app_command::{commands::cursor_move, AppCommand},
    config::AppKeybindConfig,
    error::{AppError, AppErrorCode, AppResult},
    state::AppState,
    ui::AppBackend,
};

/// Implemented by commands that accept a numeric count prefix (e.g. `5j` to move down 5 lines).
pub trait NumberPrefixCommand {
    /// Executes the command `number_prefix` times, or scaled by it.
    fn numbered_execute(
        &self,
        number_prefix: usize,
        app_state: &mut AppState,
        backend: &mut AppBackend,
        keybindings: &AppKeybindConfig,
    ) -> AppResult;
}

// In joshuto you can prefix simple commands with numbers by entering number,
// and then pressing key which some command is bound to. This is used mainly
// for easier navigation. You don't have to implement this for every command
impl NumberPrefixCommand for AppCommand {
    #[allow(unused)] // backend and keymap_t args are not used, but they probably will be
    fn numbered_execute(
        &self,
        number_prefix: usize,
        app_state: &mut AppState,
        backend: &mut AppBackend,
        keybindings: &AppKeybindConfig,
    ) -> AppResult {
        match self {
            Self::CursorMoveUp { .. } => cursor_move::up(app_state, number_prefix),
            Self::CursorMoveDown { .. } => cursor_move::down(app_state, number_prefix),
            _ => Err(AppError::new(
                AppErrorCode::UnrecognizedCommand,
                "Command cannot be prefixed by a number".to_string(),
            )),
        }
    }
}
