use crate::commands::KeyCommand;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::views::TuiTextField;
use crate::ui::TuiBackend;

use super::JoshutoRunnable;

pub fn readline(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    prefix: &str,
    suffix: &str,
) -> JoshutoResult<()> {
    context.flush_event();
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
