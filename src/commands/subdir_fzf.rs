use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::config::clean::app::search::CaseSensitivity;
use crate::context::AppContext;
use crate::error::AppResult;
use crate::ui::AppBackend;

use super::change_directory::change_directory;

pub fn subdir_fzf(context: &mut AppContext, backend: &mut AppBackend) -> AppResult {
    backend.terminal_drop();

    let mut cmd = Command::new("fzf");
    cmd.stdout(Stdio::piped());

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

    let fzf = cmd.spawn()?;

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

pub fn fzf_change_dir(context: &mut AppContext, path: &Path) -> AppResult {
    if path.is_dir() {
        change_directory(context, path)?;
    } else if let Some(parent) = path.parent() {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap()
            .trim();
        change_directory(context, parent)?;

        let index = match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
            Some(curr_list) => curr_list
                .iter()
                .enumerate()
                .find(|(_, e)| e.file_name() == file_name)
                .map(|(i, _)| i),
            None => None,
        };

        if let Some(index) = index {
            let ui_context = context.ui_context_ref().clone();
            let display_options = context.config_ref().display_options_ref().clone();
            if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
                curr_list.set_index(Some(index), &ui_context, &display_options);
            }
        }
    }
    Ok(())
}
