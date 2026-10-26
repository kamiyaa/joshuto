use std::str::FromStr;

use rustyline::history::History;

use crate::app_command::AppCommand;
use crate::app_command::ExecutableAppCommand;
use crate::config::AppKeybindConfig;
use crate::error::AppResult;
use crate::state::AppState;
use crate::ui::views::{DummyListener, TuiTextField};
use crate::ui::AppBackend;

/// Implements the `:` command line: prompts for input (pre/post-filled with `prefix`/`suffix`),
/// records it in history, expands any matching alias, and parses and executes the result.
pub fn read_and_execute(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    keymap_t: &AppKeybindConfig,
    prefix: &str,
    suffix: &str,
) -> AppResult {
    app_state.flush_event();

    let mut listener = DummyListener {};
    let user_input: Option<String> = TuiTextField::default()
        .prompt(":")
        .prefix(prefix)
        .suffix(suffix)
        .get_input(app_state, backend, &mut listener);

    if let Some(mut s) = user_input {
        let mut trimmed = s.trim_start();
        let _ = app_state.state.commandline_state.history.add(trimmed);

        let (command, arg) = match trimmed.find(' ') {
            Some(i) => (&trimmed[..i], &trimmed[i..]),
            None => (trimmed, ""),
        };

        if let Some(alias) = app_state.config.cmd_aliases.get(trimmed) {
            trimmed = alias;
        } else if let Some(alias) = app_state.config.cmd_aliases.get(command) {
            s.replace_range(..s.len() - arg.len(), alias);
            trimmed = &s;
        }

        let command = AppCommand::from_str(trimmed)?;
        command.execute(app_state, backend, keymap_t)
    } else {
        Ok(())
    }
}
