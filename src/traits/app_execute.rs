use crate::error::AppResult;
use crate::types::completion_kind::CompletionKind;
use crate::types::keymap::AppKeyMapping;
use crate::types::state::AppState;
use crate::ui::AppBackend;

pub trait AppExecute {
    fn execute(
        &self,
        app_state: &mut AppState,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult;
}

pub trait NumberedExecute {
    fn numbered_execute(
        &self,
        number_prefix: usize,
        app_state: &mut AppState,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult;
}

pub trait InteractiveExecute {
    fn interactive_execute(&self, app_state: &mut AppState);
}

pub trait AppCommand: AppExecute + std::fmt::Display + std::fmt::Debug {
    fn command(&self) -> &'static str;
}

pub trait CommandComment {
    fn comment(&self) -> &'static str;
}

pub trait CommandCompletion {
    fn completion_kind<'a>(cmd: &'a str) -> Option<CompletionKind<'a>>;
}
