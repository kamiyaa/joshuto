use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::commands::cursor_move;
use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::TuiBackend;

pub fn search_fzf(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let items = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|list| {
            let v: Vec<String> = list
                .iter()
                .enumerate()
                .map(|(i, entry)| format!("{} {}\n", i, entry.file_name().to_string()))
                .collect();
            v
        })
        .unwrap_or_else(Vec::new);

    if items.is_empty() {
        return Err(JoshutoError::new(
            JoshutoErrorKind::Io(io::ErrorKind::InvalidData),
            "no files to select".to_string(),
        ));
    }

    backend.terminal_drop();

    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    match fzf.stdin.as_mut() {
        Some(fzf_stdin) => {
            let mut writer = io::BufWriter::new(fzf_stdin);
            for item in items {
                writer.write_all(item.as_bytes())?;
            }
        }
        None => {}
    }
    let fzf_output = fzf.wait_with_output();

    backend.terminal_restore()?;

    if let Ok(output) = fzf_output {
        if output.status.success() {
            if let Ok(selected) = std::str::from_utf8(&output.stdout) {
                let selected_idx_str = selected.split_once(' ');
                if let Some((selected_idx_str, _)) = selected_idx_str {
                    if let Ok(index) = selected_idx_str.parse::<usize>() {
                        cursor_move::cursor_move(index, context);
                    }
                }
            }
        }
    }

    Ok(())
}
