use std;
use std::process;

use joshuto::context::JoshutoContext;
use joshuto::command::JoshutoCommand;
use joshuto::command::JoshutoRunnable;
use joshuto::ui;

#[derive(Clone, Debug)]
pub struct Quit;

impl Quit {
    pub fn new() -> Self { Quit }
    pub fn command() -> &'static str { "quit" }
}

impl JoshutoCommand for Quit {}

impl std::fmt::Display for Quit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for Quit {
    fn execute(&self, _: &mut JoshutoContext)
    {
        ui::end_ncurses();
        process::exit(0);
    }
}
