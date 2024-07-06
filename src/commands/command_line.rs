use std::str::FromStr;

use crate::config::clean::keymap::AppKeyMapping;
use crate::context::AppContext;
use crate::error::AppResult;
use crate::key_command::{AppExecute, Command};
use crate::ui::views::{DummyListener, TuiTextField};
use crate::ui::AppBackend;

pub fn read_and_execute(
    context: &mut AppContext,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
    prefix: &str,
    suffix: &str,
) -> AppResult {
    context.flush_event();
    let mut listener = DummyListener {};
    let user_input: Option<String> = TuiTextField::default()
        .prompt(":")
        .prefix(prefix)
        .suffix(suffix)
        .get_input(backend, context, &mut listener);

    if let Some(mut s) = user_input {
        let mut trimmed = s.trim_start();
        let _ = context.commandline_context_mut().history_mut().add(trimmed);

        let (command, arg) = match trimmed.find(' ') {
            Some(i) => (&trimmed[..i], &trimmed[i..]),
            None => (trimmed, ""),
        };

        if let Some(alias) = context.config_ref().cmd_aliases.get(trimmed) {
            trimmed = alias;
        } else if let Some(alias) = context.config_ref().cmd_aliases.get(command) {
            s.replace_range(..s.len() - arg.len(), alias);
            trimmed = &s;
        }

        let command = Command::from_str(trimmed)?;
        command.execute(context, backend, keymap_t)
    } else {
        Ok(())
    }
}
