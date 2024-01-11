use crate::context::AppContext;
use crate::error::AppResult;
use crate::ui::AppBackend;
use std::process::{Command, Stdio};

use super::reload;

#[derive(Debug, Clone)]
pub enum SubprocessCallMode {
    Interactive,
    Spawn,
    Capture,
}

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
    mode: SubprocessCallMode,
) -> std::io::Result<()> {
    let mut command = Command::new(words[0].clone());
    for word in words.iter().skip(1) {
        match (*word).as_str() {
            "%s" => {
                current_filenames(context).into_iter().for_each(|x| {
                    command.arg(x);
                });
            }
            "%p" => {
                if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
                    let mut i = 0;
                    curr_list
                        .iter_selected()
                        .map(|e| e.file_path())
                        .for_each(|file_path| {
                            command.arg(file_path);
                            i += 1;
                        });
                    if i == 0 {
                        if let Some(entry) = curr_list.curr_entry_ref() {
                            command.arg(entry.file_path());
                        }
                    }
                }
            }
            s => {
                command.arg(s);
            }
        };
    }
    match mode {
        SubprocessCallMode::Interactive => {
            let status = command.status();
            match status {
                Ok(status) => {
                    if status.code() == Some(0) {
                        Ok(())
                    } else {
                        Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Command failed with {:?}", status),
                        ))
                    }
                }
                Err(err) => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Shell execution failed: {:?}", err),
                )),
            }
        }
        SubprocessCallMode::Spawn => {
            command
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;
            Ok(())
        }
        SubprocessCallMode::Capture => {
            let output = command
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()?;
            if output.status.code() == Some(0) {
                context.last_stdout = Some(String::from_utf8_lossy(&output.stdout).to_string());
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Command failed with {:?}", output.status),
                ))
            }
        }
    }
}

/// Handler for Joshuto's `shell` and `spawn` commands.
pub fn sub_process(
    context: &mut AppContext,
    backend: &mut AppBackend,
    words: &[String],
    mode: SubprocessCallMode,
) -> AppResult {
    match mode {
        SubprocessCallMode::Interactive => {
            // Joshuto needs to release the terminal when handing it over to some interactive
            // shell command and restore it afterwards
            backend.terminal_drop();
            execute_sub_process(context, words, mode)?;
            backend.terminal_restore(context.config_ref().mouse_support)?;
            let _ = reload::soft_reload_curr_tab(context);
            context
                .message_queue_mut()
                .push_info(format!("Finished: {}", words.join(" ")));
        }
        SubprocessCallMode::Spawn => {
            execute_sub_process(context, words, mode)?;
            context
                .message_queue_mut()
                .push_info(format!("Spawned: {}", words.join(" ")));
        }
        SubprocessCallMode::Capture => {
            execute_sub_process(context, words, mode)?;
            let _ = reload::soft_reload_curr_tab(context);
        }
    };
    Ok(())
}
