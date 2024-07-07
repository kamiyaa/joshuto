use crate::error::{AppError, AppErrorKind, AppResult};
use crate::types::state::AppState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuitAction {
    DoNot,
    Noop,
    Force,
    OutputCurrentDirectory,
    OutputSelectedFiles,
}

impl QuitAction {
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
