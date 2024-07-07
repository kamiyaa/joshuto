use std::str::FromStr;

use crate::error::AppResult;
use crate::traits::app_execute::AppExecute;
use crate::types::command::Command;
use crate::types::keymap::AppKeyMapping;
use crate::types::state::AppState;
use crate::ui::views::{DummyListener, TuiTextField};
use crate::ui::AppBackend;

pub fn read_and_execute(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
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
        let _ = app_state
            .state
            .commandline_state_mut()
            .history_mut()
            .add(trimmed);

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

        let command = Command::from_str(trimmed)?;
        command.execute(app_state, backend, keymap_t)
    } else {
        Ok(())
    }
}
