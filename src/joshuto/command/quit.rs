extern crate fs_extra;
extern crate ncurses;

use std;
use std::fmt;
use std::process;

use joshuto;
use joshuto::command;

#[derive(Debug)]
pub struct Quit;

impl Quit {
    pub fn new() -> Self
    {
        Quit
    }
    fn command() -> &'static str
    {
        "Quit"
    }
}

impl command::JoshutoCommand for Quit {}

impl std::fmt::Display for Quit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for Quit {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        ncurses::endwin();
        process::exit(0);
    }
}
