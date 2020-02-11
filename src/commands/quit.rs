use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::TuiBackend;

#[derive(Clone, Debug)]
pub struct Quit;

impl Quit {
    pub fn new() -> Self {
        Self::default()
    }
    pub const fn command() -> &'static str {
        "quit"
    }

    pub fn quit(context: &mut JoshutoContext) -> JoshutoResult<()> {
        if !context.worker_queue.is_empty() {
            Err(JoshutoError::new(
                JoshutoErrorKind::IOOther,
                String::from("operations running in background, use force_quit to quit"),
            ))
        } else {
            context.exit = true;
            Ok(())
        }
    }
}

impl JoshutoCommand for Quit {}

impl std::fmt::Display for Quit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for Quit {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        Self::quit(context)
    }
}

impl std::default::Default for Quit {
    fn default() -> Self {
        Quit
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
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        Self::force_quit(context);
        Ok(())
    }
}
