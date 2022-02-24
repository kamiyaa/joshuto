use std::str::FromStr;

use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::key_command::{AppExecute, Command};
use crate::ui::views::TuiTextField;
use crate::ui::TuiBackend;

pub fn read_and_execute(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    keymap_t: &AppKeyMapping,
    prefix: &str,
    suffix: &str,
) -> JoshutoResult {
    context.flush_event();
    let user_input: Option<String> = TuiTextField::default()
        .prompt(":")
        .prefix(prefix)
        .suffix(suffix)
        .get_input(backend, context);

    if let Some(s) = user_input {
        let trimmed = s.trim_start();
        context.commandline_context_mut().history_mut().add(trimmed);
        let command = Command::from_str(trimmed)?;
        command.execute(context, backend, keymap_t)
    } else {
        Ok(())
    }
}
