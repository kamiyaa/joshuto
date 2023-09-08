use std::io;

use crate::context::AppContext;
use crate::error::{AppError, AppErrorKind, AppResult};

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

pub fn quit_with_action(context: &mut AppContext, quit_action: QuitAction) -> AppResult {
    if quit_action == QuitAction::Force {
        context.quit = quit_action;
        return Ok(());
    }

    let worker_context = context.worker_context_ref();
    if worker_context.is_busy() || !worker_context.is_empty() {
        Err(AppError::new(
            AppErrorKind::Io(io::ErrorKind::Other),
            String::from("operations running in background, use `quit --force` to quit"),
        ))
    } else {
        context.quit = quit_action;
        Ok(())
    }
}
