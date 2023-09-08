use std::path::{Path, PathBuf};

use crate::context::AppContext;
use crate::error::AppResult;
use crate::ui::AppBackend;

use super::change_directory::change_directory;
use super::fzf;

pub fn subdir_fzf(context: &mut AppContext, backend: &mut AppBackend) -> AppResult {
    let fzf_output = fzf::fzf(context, backend, Vec::new())?;
    let path: PathBuf = PathBuf::from(fzf_output);
    fzf_change_dir(context, path.as_path())
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
