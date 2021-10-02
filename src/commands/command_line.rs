use std::str::FromStr;

use crate::commands::KeyCommand;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::views::TuiTextField;
use crate::ui::TuiBackend;

use super::AppExecute;

pub fn read_and_execute(
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
        context.commandline_context_mut().history_mut().add(trimmed);
        let command = KeyCommand::from_str(trimmed)?;
        command.execute(context, backend)
    } else {
        Ok(())
    }
}
