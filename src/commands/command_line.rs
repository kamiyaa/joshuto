use std::str::FromStr;

use crate::commands::KeyCommand;
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::views::TuiTextField;
use crate::ui::TuiBackend;

use super::AppExecute;

pub fn readline(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    keymap_t: &AppKeyMapping,
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
        let command = KeyCommand::from_str(trimmed)?;
        command.execute(context, backend, keymap_t)
    } else {
        Ok(())
    }
}
