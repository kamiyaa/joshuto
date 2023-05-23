use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::commands::cursor_move;
use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::AppBackend;

use super::change_directory::change_directory;
use super::search_glob::search_glob;

pub fn search_fzf(
    context: &mut AppContext,
    backend: &mut AppBackend,
    fzf_rec: bool,
) -> JoshutoResult {
    let items = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|list| {
            let v: Vec<String> = list
                .iter()
                .enumerate()
                .map(|(i, entry)| format!("{} {}\n", i, entry.file_name()))
                .collect();
            v
        })
        .unwrap_or_default();

    if items.is_empty() {
        return Err(JoshutoError::new(
            JoshutoErrorKind::Io(io::ErrorKind::InvalidData),
            "no files to select".to_string(),
        ));
    }

    backend.terminal_drop();

    let mut fzf_command = Command::new("fzf");

    if !fzf_rec {
        fzf_command.stdin(Stdio::piped());
    };
    fzf_command.stdout(Stdio::piped());

    let mut fzf = match fzf_command.spawn() {
        Ok(child) => child,
        Err(e) => {
            backend.terminal_restore()?;
            return Err(JoshutoError::from(e));
        }
    };

    if let Some(fzf_stdin) = fzf.stdin.as_mut() {
        let mut writer = io::BufWriter::new(fzf_stdin);
        for item in items {
            writer.write_all(item.as_bytes())?;
        }
    }
    let fzf_output = fzf.wait_with_output();

    backend.terminal_restore()?;

    if let Ok(output) = fzf_output {
        if output.status.success() {
            if let Ok(selected) = std::str::from_utf8(&output.stdout) {
                if fzf_rec {
                    let new_path = selected.rsplit_once("/");
                    if let Some((new_path, file_name)) = new_path {
                        // cd to new path
                        change_directory(context, &Path::new(new_path))?;
                        // Set cursor to new index
                        let len = file_name.len();
                        search_glob(context, &file_name[0..len - 1])?;
                    }
                } else {
                    let selected_idx_str = selected.split_once(' ');
                    if let Some((selected_idx_str, _)) = selected_idx_str {
                        if let Ok(index) = selected_idx_str.parse::<usize>() {
                            cursor_move::cursor_move(context, index);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
