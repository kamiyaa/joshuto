use std::path;

use crate::context::AppContext;
use crate::fs::JoshutoMetadata;
use crate::preview::{preview_dir, preview_file};
use crate::ui::TuiBackend;

pub fn load_preview_path(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    p: path::PathBuf,
    metadata: JoshutoMetadata,
) {
    let preview_options = context.config_ref().preview_options_ref();

    if metadata.is_dir() {
        let need_to_load = context
            .tab_context_ref()
            .curr_tab_ref()
            .history_ref()
            .get(p.as_path())
            .map(|e| e.need_update())
            .unwrap_or(true);

        if need_to_load {
            preview_dir::Background::load_preview(context, p);
        }
    } else if metadata.len() <= preview_options.max_preview_size {
        let need_to_load = context
            .preview_context_ref()
            .get_preview(p.as_path())
            .is_none();

        if need_to_load {
            preview_file::Background::preview_path_with_script(context, backend, p);
        }
    } else {
    }
}

pub fn load_preview(context: &mut AppContext, backend: &mut TuiBackend) {
    let mut load_list = Vec::with_capacity(2);

    let curr_tab = context.tab_context_ref().curr_tab_ref();
    match curr_tab.curr_list_ref() {
        Some(curr_list) => {
            if let Some(index) = curr_list.index {
                let entry = &curr_list.contents[index];
                load_list.push((entry.file_path().to_path_buf(), entry.metadata.clone()));
            }
        }
        None => {
            if let Ok(metadata) = JoshutoMetadata::from(curr_tab.cwd()) {
                load_list.push((curr_tab.cwd().to_path_buf(), metadata));
            }
        }
    }

    for (path, metadata) in load_list {
        load_preview_path(context, backend, path, metadata);
    }
}
