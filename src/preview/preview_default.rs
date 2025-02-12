use std::path;

use crate::error::AppResult;
use crate::fs::JoshutoMetadata;
use crate::preview::preview_dir;
use crate::types::state::AppState;
use crate::ui::AppBackend;

pub fn load_previews(app_state: &mut AppState, backend: &mut AppBackend) {
    let previews_to_load = {
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
                if let Ok(metadata) = JoshutoMetadata::from(curr_tab.get_cwd()) {
                    load_list.push((curr_tab.get_cwd().to_path_buf(), metadata));
                }
            }
        }
        load_list
    };

    for (path, metadata) in previews_to_load {
        let _ = load_preview_path(app_state, backend, path, metadata);
    }
}

pub fn load_preview_path(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    p: path::PathBuf,
    metadata: JoshutoMetadata,
) -> AppResult {
    let preview_options = &app_state.config.preview_options;
    if metadata.is_dir() {
        let tab = app_state.state.tab_state_ref().curr_tab_ref();
        // only load if there doesn't already exist a loading thread and
        // there isn't an entry in history
        let need_to_load = tab
            .history_metadata_ref()
            .get(p.as_path())
            .map(|m| m.is_loading())
            .unwrap_or(true)
            && tab
                .history_ref()
                .get(p.as_path())
                .map(|e| e.need_update())
                .unwrap_or(true);

        if need_to_load {
            preview_dir::Background::load_preview(app_state, p);
        }
    } else if metadata.len() <= preview_options.max_preview_size {
        app_state
            .state
            .preview_state_mut()
            .load_preview_lazy(&app_state.config, backend, p)?;
    }
    Ok(())
}
