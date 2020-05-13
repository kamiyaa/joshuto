use std::process;

use crate::commands::{self, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::widgets::TuiTextField;
use crate::ui::TuiBackend;

#[derive(Clone, Debug)]
pub struct ShellCommand {
    pub command: String,
}

impl ShellCommand {
    pub fn new(command: String) -> Self {
        Self { command }
    }
    pub const fn command() -> &'static str {
        "console"
    }

    pub fn shell_command(command: &str) -> std::io::Result<()> {
        let mut command = process::Command::new("sh").arg("-c").arg(command).spawn()?;
        Ok(())
    }
}

impl JoshutoCommand for ShellCommand {}

impl std::fmt::Display for ShellCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: sh -c '{}'", Self::command(), self.command)
    }
}

impl JoshutoRunnable for ShellCommand {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        backend.terminal_drop();
        let res = Self::shell_command(self.command.as_str());
        backend.terminal_restore()?;
        res?;
        Ok(())
    }
}
