use crate::context::AppContext;
use crate::error::AppResult;
use crate::ui::AppBackend;
use std::path::Path;
use std::process::{Command, Stdio};

use super::reload;

#[derive(Debug, Clone)]
pub enum SubprocessCallMode {
    Interactive,
    Spawn,
    Capture,
}

pub fn current_files(context: &AppContext) -> Vec<(&str, &Path)> {
    let mut result = Vec::new();
    if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        let mut i = 0;
        curr_list.iter_selected().for_each(|entry| {
            result.push((entry.file_name(), entry.file_path()));
            i += 1;
        });
        if i == 0 {
            if let Some(entry) = curr_list.curr_entry_ref() {
                result.push((entry.file_name(), entry.file_path()));
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
    let current_files = current_files(context);
    let command_base = if current_files.len() == 1 {
        words[0]
            .replace("%s", current_files[0].0)
            .replace("%p", &current_files[0].1.to_string_lossy())
    } else {
        words[0].clone()
    };

    let mut command = Command::new(command_base);
    for word in words.iter().skip(1) {
        match word.as_str() {
            "%s" => {
                for (file_name, _) in &current_files {
                    command.arg(file_name);
                }
            }
            "%p" => {
                for (_, file_path) in &current_files {
                    command.arg(file_path);
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
                    format!("Shell execution failed: {}", err),
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
            let res = execute_sub_process(context, words, mode);
            backend.terminal_restore()?;
            let _ = reload::soft_reload_curr_tab(context);
            res?;
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
