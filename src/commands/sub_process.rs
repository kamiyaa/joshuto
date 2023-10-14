use crate::context::AppContext;
use crate::error::AppResult;
use crate::ui::AppBackend;
use std::process::{Command, Stdio};

use super::reload;

pub fn current_filenames(context: &AppContext) -> Vec<&str> {
    let mut result = Vec::new();
    if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        let mut i = 0;
        curr_list
            .iter_selected()
            .map(|e| e.file_name())
            .for_each(|file_name| {
                result.push(file_name);
                i += 1;
            });
        if i == 0 {
            if let Some(entry) = curr_list.curr_entry_ref() {
                result.push(entry.file_name());
            }
        }
    }

    result
}

fn execute_sub_process(
    context: &mut AppContext,
    words: &[String],
    spawn: bool,
) -> std::io::Result<()> {
    let mut command = Command::new(words[0].clone());
    for word in words.iter().skip(1) {
        match (*word).as_str() {
            "%s" => {
                current_filenames(context).into_iter().for_each(|x| {
                    command.arg(x);
                });
            }
            s => {
                command.arg(s);
            }
        };
    }
    if spawn {
        command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
    } else {
        command.status()?;
    }
    Ok(())
}

/// Handler for Joshuto's `shell` and `spawn` commands.
pub fn sub_process(
    context: &mut AppContext,
    backend: &mut AppBackend,
    words: &[String],
    spawn: bool,
) -> AppResult {
    backend.terminal_drop();
    let res = execute_sub_process(context, words, spawn);
    backend.terminal_restore()?;
    let _ = reload::soft_reload_curr_tab(context);
    context.message_queue_mut().push_info(format!(
        "{}: {}",
        if spawn { "Spawned" } else { "Finished" },
        words.join(" ")
    ));
    res?;
    Ok(())
}
