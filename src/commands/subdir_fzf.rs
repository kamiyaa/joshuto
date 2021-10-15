use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
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

    backend.terminal_restore()?;

    if let Ok(output) = fzf_output {
        if output.status.success() {
            if let Ok(selected) = std::str::from_utf8(&output.stdout) {
                let path: PathBuf = PathBuf::from(selected);
                if path.is_dir() {
                    change_directory(context, &path)?;
                } else if let Some(parent) = path.parent() {
                    change_directory(context, &parent)?;
                }
            }
        }
    }

    Ok(())
}
