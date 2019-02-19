use std::process;

use commands::{JoshutoCommand, JoshutoRunnable};
use context::JoshutoContext;
use ui;

#[derive(Clone, Debug)]
pub struct Quit;

impl Quit {
    pub fn new() -> Self {
        Quit
    }
    pub const fn command() -> &'static str {
        "quit"
    }
}

impl JoshutoCommand for Quit {}

impl std::fmt::Display for Quit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for Quit {
    fn execute(&self, context: &mut JoshutoContext) {
        context.exit = true;
    }
}
