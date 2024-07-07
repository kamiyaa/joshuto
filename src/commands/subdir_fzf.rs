use std::path::{Path, PathBuf};

use crate::error::AppResult;
use crate::types::state::AppState;
use crate::ui::AppBackend;

use super::change_directory::change_directory;
use super::fzf;

pub fn subdir_fzf(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    let fzf_output = fzf::fzf(app_state, backend, Vec::new())?;
    let path: PathBuf = PathBuf::from(fzf_output);
    fzf_change_dir(app_state, path.as_path())
}

pub fn fzf_change_dir(app_state: &mut AppState, path: &Path) -> AppResult {
    if path.is_dir() {
        change_directory(app_state, path)?;
    } else if let Some(parent) = path.parent() {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap()
            .trim();
        change_directory(app_state, parent)?;

        let index = match app_state
            .state
            .tab_state_ref()
            .curr_tab_ref()
            .curr_list_ref()
        {
            Some(curr_list) => curr_list
                .iter()
                .enumerate()
                .find(|(_, e)| e.file_name() == file_name)
                .map(|(i, _)| i),
            None => None,
        };

        if let Some(index) = index {
            let ui_state = app_state.state.ui_state_ref().clone();
            let display_options = &app_state.config.display_options;
            if let Some(curr_list) = app_state
                .state
                .tab_state_mut()
                .curr_tab_mut()
                .curr_list_mut()
            {
                curr_list.set_index(Some(index), &ui_state, display_options);
            }
        }
    }
    Ok(())
}
