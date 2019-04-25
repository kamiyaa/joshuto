use crate::commands::{CursorMoveInc, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::window::JoshutoView;

#[derive(Debug, Clone)]
pub struct SelectFiles {
    toggle: bool,
    all: bool,
}

impl SelectFiles {
    pub fn new(toggle: bool, all: bool) -> Self {
        SelectFiles { toggle, all }
    }
    pub const fn command() -> &'static str {
        "select_files"
    }
}

impl JoshutoCommand for SelectFiles {}

impl std::fmt::Display for SelectFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} toggle={} all={}",
            Self::command(),
            self.toggle,
            self.all
        )
    }
}

impl JoshutoRunnable for SelectFiles {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        if self.toggle && !self.all {
            let curr_list = &mut context.tabs[context.curr_tab_index].curr_list;
            if let Some(s) = curr_list.get_curr_mut() {
                s.selected = !s.selected;
                return CursorMoveInc::new(1).execute(context, view);
            }
        }
        return Ok(());
    }
}
