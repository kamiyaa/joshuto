use std;
use std::fmt;
use std::process;

use joshuto;
use joshuto::command;
use joshuto::ui;

#[derive(Debug)]
pub struct Quit;

impl Quit {
    pub fn new() -> Self { Quit }
    pub fn command() -> &'static str { "quit" }
}

impl command::JoshutoCommand for Quit {}

impl std::fmt::Display for Quit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl command::Runnable for Quit {
    fn execute(&self, _: &mut joshuto::JoshutoContext)
    {
        ui::end_ncurses();
        process::exit(0);
    }
}
