use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;
use std::process::{Command, Stdio};

use super::reload;

fn shell_command(context: &mut AppContext, words: &[String], spawn: bool) -> std::io::Result<()> {
    let mut command = Command::new(words[0].clone());
    for word in words.iter().skip(1) {
        match (*word).as_str() {
            "%s" => {
                if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
                    let mut i = 0;
                    for entry in curr_list.iter_selected().map(|e| e.file_name()) {
                        command.arg(entry);
                        i += 1;
                    }
                    if i == 0 {
                        if let Some(entry) = curr_list.curr_entry_ref() {
                            command.arg(entry.file_name());
                        }
                    }
                }
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

pub fn shell(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    words: &[String],
    spawn: bool,
) -> JoshutoResult<()> {
    backend.terminal_drop();
    let res = shell_command(context, words, spawn);
    reload::soft_reload(context.tab_context_ref().index, context)?;
    context.push_msg(format!(
        "{}: {}",
        if spawn { "Spawned" } else { "Finished" },
        words.join(" ")
    ));
    backend.terminal_restore()?;
    res?;
    Ok(())
}
