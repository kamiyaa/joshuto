use std::str::FromStr;

use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::key_command::{AppExecute, Command};
use crate::ui::views::{DummyListener, TuiTextField};
use crate::ui::AppBackend;

pub fn read_and_execute(
    context: &mut AppContext,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
    prefix: &str,
    suffix: &str,
) -> JoshutoResult {
    context.flush_event();
    let mut listener = DummyListener {};
    let user_input: Option<String> = TuiTextField::default()
        .prompt(":")
        .prefix(prefix)
        .suffix(suffix)
        .get_input(backend, context, &mut listener);

    if let Some(s) = user_input {
        let mut trimmed = s.trim_start();
        let _ = context.commandline_context_mut().history_mut().add(trimmed);

        if let Some(alias) = context.config_ref().cmd_aliases.get(trimmed) {
            trimmed = alias;
        }

        let command = Command::from_str(trimmed)?;
        command.execute(context, backend, keymap_t)
    } else {
        Ok(())
    }
}
