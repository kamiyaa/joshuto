use crate::error::AppResult;
use crate::types::completion_kind::CompletionKind;
use crate::types::keymap::AppKeyMapping;
use crate::types::state::AppState;
use crate::ui::AppBackend;

/// Implemented by every command type; runs the command against the current app state.
pub trait AppExecute {
    /// Executes the command, mutating `app_state` and/or `backend` as needed.
    fn execute(
        &self,
        app_state: &mut AppState,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult;
}

/// Implemented by commands that accept a numeric count prefix (e.g. `5j` to move down 5 lines).
pub trait NumberedExecute {
    /// Executes the command `number_prefix` times, or scaled by it.
    fn numbered_execute(
        &self,
        number_prefix: usize,
        app_state: &mut AppState,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult;
}

/// Implemented by commands that run interactively, reading further input as they execute.
pub trait InteractiveExecute {
    /// Runs the command's interactive loop against `app_state`.
    fn interactive_execute(&self, app_state: &mut AppState);
}

/// Implemented by top-level command enums that can be looked up and displayed by name.
pub trait AppCommand: AppExecute + std::fmt::Display + std::fmt::Debug {
    /// Returns the command's canonical name, as used in config files and the command line.
    fn command(&self) -> &'static str;
}

/// Implemented by commands that provide a help-text comment describing what they do.
pub trait CommandComment {
    /// Returns a short human-readable description of the command.
    fn comment(&self) -> &'static str;
}

/// Implemented by commands that support tab-completion of their arguments.
pub trait CommandCompletion {
    /// Returns the kind of completion to offer for the partially-typed command line `cmd`.
    fn completion_kind<'a>(cmd: &'a str) -> Option<CompletionKind<'a>>;
}
