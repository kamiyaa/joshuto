use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::commands::cursor_move;
use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::TuiBackend;

use super::change_directory::change_directory;

pub fn subdir_fzf(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let fzf_default_command = std::env::var("FZF_DEFAULT_COMMAND")?;

    backend.terminal_drop();

    let fzf_default_command_output = {
        let mut fzf_results = Command::new("bash")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        match fzf_results.stdin.as_mut() {
            Some(fzf_stdin) => {
                let mut writer = io::BufWriter::new(fzf_stdin);
                writer.write_all(fzf_default_command.as_bytes())?;
            }
            None => {}
        }
        fzf_results.wait_with_output()?
    };

    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    match fzf.stdin.as_mut() {
        Some(fzf_stdin) => {
            let mut writer = io::BufWriter::new(fzf_stdin);
            writer.write_all(&fzf_default_command_output.stdout)?;
        }
        None => {}
    }
    let fzf_output = fzf.wait_with_output();

    match fzf_output {
        Ok(output) if output.status.success() => {
            if let Ok(selected) = std::str::from_utf8(&output.stdout) {
                let path: PathBuf = PathBuf::from(selected);
                fzf_change_dir(context, path.as_path())?;
            }
        }
        _ => {}
    }

    backend.terminal_restore()?;

    Ok(())
}

pub fn fzf_change_dir(context: &mut AppContext, path: &Path) -> JoshutoResult<()> {
    if path.is_dir() {
        change_directory(context, &path)?;
    } else if let Some(parent) = path.parent() {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap()
            .trim();

        change_directory(context, &parent)?;

        let index = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
            Some(curr_list) => curr_list
                .iter()
                .enumerate()
                .find(|(i, e)| e.file_name() == file_name)
                .map(|(i, e)| i),
            None => None,
        };
        eprintln!("{:?}", index);

        if let Some(index) = index {
            if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
                curr_list.index = Some(index);
            }
        }
    }
    Ok(())
}
