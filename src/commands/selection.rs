use commands::{CursorMove, JoshutoCommand, JoshutoRunnable};
use context::JoshutoContext;

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
    fn execute(&self, context: &mut JoshutoContext) {
        if self.toggle && !self.all {
            let mut selected = false;

            if let Some(s) = context.tabs[context.curr_tab_index].curr_list.as_mut() {
                s.curr_toggle_select();
                selected = true;
            }
            if selected {
                let subcommand = CursorMove::new(1);
                subcommand.execute(context);
            }
        }
    }
}
