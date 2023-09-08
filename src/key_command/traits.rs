use crate::config::clean::keymap::AppKeyMapping;
use crate::context::AppContext;
use crate::error::AppResult;
use crate::ui::AppBackend;

pub trait AppExecute {
    fn execute(
        &self,
        context: &mut AppContext,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult;
}

pub trait NumberedExecute {
    fn numbered_execute(
        &self,
        number_prefix: usize,
        context: &mut AppContext,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult;
}

pub trait InteractiveExecute {
    fn interactive_execute(&self, context: &mut AppContext);
}

pub trait AppCommand: AppExecute + std::fmt::Display + std::fmt::Debug {
    fn command(&self) -> &'static str;
}

pub trait CommandComment {
    fn comment(&self) -> &'static str;
}
