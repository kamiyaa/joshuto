use std::path;

use crate::context::AppContext;
use crate::preview::{preview_dir, preview_file};
use crate::ui::TuiBackend;

pub fn load_preview_path(context: &mut AppContext, backend: &mut TuiBackend, p: path::PathBuf) {
    if p.is_dir() {
        preview_dir::Background::load_preview(context, p);
    } else if p.is_file() {
        preview_file::Background::preview_path_with_script(context, backend, p);
    }
}

pub fn load_preview(context: &mut AppContext, backend: &mut TuiBackend) {
    let mut p: Option<path::PathBuf> = None;
    if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        if let Some(index) = curr_list.index {
            let entry = &curr_list.contents[index];
            p = Some(entry.file_path().to_path_buf())
        }
    }

    if let Some(p) = p {
        load_preview_path(context, backend, p);
    }
}
