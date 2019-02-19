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

    pub fn quit(context: &mut JoshutoContext) {
        if !context.threads.is_empty() {
            ui::wprint_err(
                &context.views.bot_win,
                "Error: operations running in background, use force_quit to quit",
            );
            ncurses::doupdate();
            return;
        }
        context.exit = true;
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
        Self::quit(context);
    }
}

#[derive(Clone, Debug)]
pub struct ForceQuit;

impl ForceQuit {
    pub fn new() -> Self {
        ForceQuit
    }
    pub const fn command() -> &'static str {
        "force_quit"
    }

    pub fn force_quit(context: &mut JoshutoContext) {
        context.exit = true;
    }
}

impl JoshutoCommand for ForceQuit {}

impl std::fmt::Display for ForceQuit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for ForceQuit {
    fn execute(&self, context: &mut JoshutoContext) {
        Self::force_quit(context);
    }
}
