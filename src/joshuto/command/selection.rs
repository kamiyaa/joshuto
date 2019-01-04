extern crate fs_extra;
extern crate ncurses;

use std;
use std::fmt;

use joshuto;
use joshuto::command;

#[derive(Debug)]
pub struct SelectFiles {
    toggle: bool,
    all: bool,
}

impl SelectFiles {
    pub fn new(toggle: bool, all: bool) -> Self
    {
        SelectFiles {
            toggle,
            all,
        }
    }
    pub const fn command() -> &'static str { "select_files" }
}

impl command::JoshutoCommand for SelectFiles {}

impl std::fmt::Display for SelectFiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} toggle={} all={}", Self::command(), self.toggle, self.all)
    }
}

impl command::Runnable for SelectFiles {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if self.toggle && !self.all {
            let mut selected = false;
            if let Some(s) = context.curr_list.as_mut() {
                s.curr_toggle_select();
                selected = true;
            }
            if selected {
                let subcommand = command::CursorMove::new(1);
                subcommand.execute(context);
            }
        }
    }
}
