use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::commands::cursor_move;
use crate::config::clean::app::search::CaseSensitivity;
use crate::context::AppContext;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::ui::AppBackend;

pub fn search_fzf(context: &mut AppContext, backend: &mut AppBackend) -> AppResult {
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
        return Err(AppError::new(
            AppErrorKind::Io(io::ErrorKind::InvalidData),
            "no files to select".to_string(),
        ));
    }

    backend.terminal_drop();

    let mut cmd = Command::new("fzf");
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let case_sensitivity = context
        .config_ref()
        .search_options_ref()
        .fzf_case_sensitivity;

    match case_sensitivity {
        CaseSensitivity::Insensitive => {
            cmd.arg("-i");
        }
        CaseSensitivity::Sensitive => {
            cmd.arg("+i");
        }
        // fzf uses smart-case match by default
        CaseSensitivity::Smart => {}
    }

    let mut fzf = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            backend.terminal_restore()?;
            return Err(AppError::from(e));
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
                let selected_idx_str = selected.split_once(' ');
                if let Some((selected_idx_str, _)) = selected_idx_str {
                    if let Ok(index) = selected_idx_str.parse::<usize>() {
                        cursor_move::cursor_move(context, index);
                    }
                }
            }
        }
    }

    Ok(())
}
