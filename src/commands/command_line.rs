use crate::commands::{self, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::widgets::TuiTextField;
use crate::ui::TuiBackend;

#[derive(Clone, Debug)]
pub struct CommandLine {
    pub prefix: String,
    pub suffix: String,
}

impl CommandLine {
    pub fn new(prefix: String, suffix: String) -> Self {
        CommandLine { prefix, suffix }
    }
    pub const fn command() -> &'static str {
        "console"
    }

    pub fn readline(
        &self,
        context: &mut JoshutoContext,
        backend: &mut TuiBackend,
    ) -> JoshutoResult<()> {
        let user_input: Option<String> = TuiTextField::default()
            .prompt(":")
            .prefix(self.prefix.as_str())
            .suffix(self.suffix.as_str())
            .get_input(backend, context);

        if let Some(s) = user_input {
            let trimmed = s.trim_start();
            let command = commands::parse_command(trimmed)?;
            command.execute(context, backend)
        } else {
            Ok(())
        }
    }
}

impl JoshutoCommand for CommandLine {}

impl std::fmt::Display for CommandLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {} {}", Self::command(), self.prefix, self.suffix)
    }
}

impl JoshutoRunnable for CommandLine {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        self.readline(context, backend)
    }
}
