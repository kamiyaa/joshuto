use std::path;

use crate::{fs::AppFileMetadata, state::AppState, tasks::preview::preview_dir, ui::AppBackend};

/// Loads a preview for the entry currently under the cursor (or the current directory, if
/// empty) in the active tab.
pub fn load_previews(app_state: &mut AppState, backend: &mut AppBackend) {
    let mut load_list = Vec::with_capacity(2);

    let curr_tab = app_state.state.tab_state_ref().curr_tab_ref();
    match curr_tab.curr_list_ref() {
        Some(curr_list) => {
            if let Some(index) = curr_list.get_index() {
                let entry = &curr_list.contents[index];
                load_list.push((entry.file_path().to_path_buf(), entry.metadata.clone()));
            }
        }
        None => {
            if let Ok(metadata) = AppFileMetadata::from(curr_tab.get_cwd()) {
                load_list.push((curr_tab.get_cwd().to_path_buf(), metadata));
            }
        }
    }

    for (path, metadata) in load_list {
        load_preview_path(app_state, backend, path, metadata);
    }
}

/// Loads a preview for `p`: kicks off a background directory read if it's a directory not
/// already loading or cached, or loads a file preview if it's under the configured size limit.
pub fn load_preview_path(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    p: path::PathBuf,
    metadata: AppFileMetadata,
) {
    let preview_options = &app_state.config.preview_options;
    if metadata.is_dir() {
        let tab = app_state.state.tab_state_ref().curr_tab_ref();
        // only if there isn't already a loading thread or a failed load; whether a cached
        // listing is out of date is checked by the loading thread, to keep IO off this one
        if tab.history_metadata_ref().get(p.as_path()).is_none() {
            preview_dir::Background::refresh(app_state, p);
        }
    } else if metadata.len() <= preview_options.max_preview_size {
        app_state
            .state
            .preview_state_mut()
            .load_preview(&app_state.config, backend, p);
    }
}
