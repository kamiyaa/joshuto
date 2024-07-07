use super::change_directory::change_directory;
use super::sub_process::current_files;
use crate::commands::cursor_move;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::types::state::AppState;
use crate::ui::AppBackend;
use shell_words::split;
use std::process::{Command, Stdio};

pub fn custom_search(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    words: &[String],
    interactive: bool,
) -> AppResult {
    let custom_command = app_state
        .config
        .custom_commands
        .as_slice()
        .iter()
        .find(|x| x.name == words[0])
        .ok_or(AppError::new(
            AppErrorKind::InvalidParameters,
            "No custom command with given name".into(),
        ))?
        .command
        .clone();

    let current_filenames: Vec<&str> = current_files(app_state).iter().map(|f| f.0).collect();

    let text = custom_command.replace("%s", &current_filenames.join(" "));
    let text = text.replace(
        "%text",
        &words
            .iter()
            .skip(1)
            .cloned()
            .collect::<Vec<String>>()
            .join(" "),
    );
    let mut command_with_args: Vec<String> = split(&text).map_err(|_| {
        AppError::new(
            AppErrorKind::InvalidParameters,
            "Command cannot be splitted".into(),
        )
    })?;

    let mut cmd = Command::new(command_with_args.remove(0));
    command_with_args.into_iter().for_each(|x| {
        cmd.arg(x);
    });

    let cmd_result = if interactive {
        backend.terminal_drop();
        let cmd_result = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?;
        backend.terminal_restore()?;
        cmd_result
    } else {
        cmd.output()?
    };

    if cmd_result.status.success() {
        let returned_text = std::str::from_utf8(&cmd_result.stdout)
            .map_err(|_| {
                AppError::new(
                    AppErrorKind::Parse,
                    "Could not get command result as utf8".into(),
                )
            })?
            .trim_end();

        let path = std::path::Path::new(returned_text);
        change_directory(
            app_state,
            path.parent().ok_or(AppError::new(
                AppErrorKind::Parse,
                "Could not get parent directory".into(),
            ))?,
        )?;

        if let Some(current_dir_items) = app_state
            .state
            .tab_state_ref()
            .curr_tab_ref()
            .curr_list_ref()
        {
            let position = current_dir_items
                .iter()
                .enumerate()
                .find(|x| x.1.file_name() == path.file_name().unwrap_or_default())
                .map(|x| x.0)
                .unwrap_or_default();

            cursor_move::cursor_move(app_state, position);
        }

        Ok(())
    } else {
        let returned_text = std::str::from_utf8(&cmd_result.stderr).map_err(|_| {
            AppError::new(
                AppErrorKind::Parse,
                "Could not get command result as utf8".into(),
            )
        })?;

        Err(AppError::new(
            AppErrorKind::Parse,
            format!("Command failed: {}", returned_text),
        ))
    }
}
