use crate::commands::KeyCommand;
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::widgets::TuiTextField;
use crate::ui::TuiBackend;

use super::JoshutoRunnable;

pub fn readline(
    context: &mut JoshutoContext,
    backend: &mut TuiBackend,
    prefix: &str,
    suffix: &str,
) -> JoshutoResult<()> {
    let user_input: Option<String> = TuiTextField::default()
        .prompt(":")
        .prefix(prefix)
        .suffix(suffix)
        .get_input(backend, context);

    if let Some(s) = user_input {
        let trimmed = s.trim_start();
        let command = KeyCommand::parse_command(trimmed)?;
        command.execute(context, backend)
    } else {
        Ok(())
    }
}
