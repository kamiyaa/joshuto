use crate::error::{AppError, AppErrorKind, AppResult};
use crate::types::state::AppState;

/// How (and whether) joshuto should quit, as requested by the `quit` command's flags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuitAction {
    /// Not quitting; the default/no-op state.
    DoNot,
    /// Quitting without producing any output.
    Noop,
    /// Quit immediately, even with background operations still running.
    Force,
    /// Quit and print the current working directory (for shell `cd` integration).
    OutputCurrentDirectory,
    /// Quit and print the currently selected files.
    OutputSelectedFiles,
}

impl QuitAction {
    /// Returns the process exit code corresponding to this quit action.
    pub const fn exit_code(&self) -> i32 {
        match *self {
            Self::Noop => 0,
            Self::DoNot => 10,
            Self::Force => 100,
            Self::OutputCurrentDirectory => 101,
            Self::OutputSelectedFiles => 102,
        }
    }
}

/// Implements `quit`: requests the main loop exit with `quit_action`, refusing to quit
/// non-forcefully while background IO tasks are pending or running.
pub fn quit_with_action(app_state: &mut AppState, quit_action: QuitAction) -> AppResult {
    if quit_action == QuitAction::Force {
        app_state.quit = quit_action;
        return Ok(());
    }

    let worker_state = app_state.state.worker_state_ref();
    if worker_state.is_busy() || !worker_state.is_empty() {
        Err(AppError::new(
            AppErrorKind::Io,
            String::from("operations running in background, use `quit --force` to quit"),
        ))
    } else {
        app_state.quit = quit_action;
        Ok(())
    }
}
