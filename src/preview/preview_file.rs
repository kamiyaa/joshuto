use std::path;
use std::process::{Command, Output};
use std::thread;
use std::time;

use crate::context::AppContext;
use crate::event::AppEvent;

#[derive(Clone, Debug)]
pub struct FilePreview {
    pub status: std::process::ExitStatus,
    pub output: String,
    pub index: usize,
    pub modified: time::SystemTime,
}

impl std::convert::From<Output> for FilePreview {
    fn from(output: Output) -> Self {
        let s = String::from_utf8_lossy(&output.stdout).to_string();
        let s2 = s.replace('\t', "        ");
        let status = output.status;
        let modified = time::SystemTime::now();
        Self {
            status,
            output: s2,
            modified,
            index: 0,
        }
    }
}

pub struct Background {}

impl Background {
    pub fn preview_path_with_script(context: &AppContext, path: path::PathBuf) {
        let preview_options = context.config_ref().preview_options_ref();
        if let Some(script) = preview_options.preview_script.as_ref() {
            let ui_context = context.ui_context_ref();
            if ui_context.layout.is_empty() {
                return;
            }
            let layout_rect = &ui_context.layout[ui_context.layout.len() - 1];

            let preview_width = layout_rect.width;
            let preview_height = layout_rect.height;
            let preview_x_coord = layout_rect.x;
            let preview_y_coord = layout_rect.y;

            let image_cache = 0;

            let script = script.clone();
            let event_tx = context.clone_event_tx();
            let _ = thread::spawn(move || {
                let file_full_path = path.as_path();

                let res = Command::new(script)
                    .arg("--path")
                    .arg(file_full_path)
                    .arg("--preview-width")
                    .arg(preview_width.to_string())
                    .arg("--preview-height")
                    .arg(preview_height.to_string())
                    .arg("--x-coord")
                    .arg(preview_x_coord.to_string())
                    .arg("--y-coord")
                    .arg(preview_y_coord.to_string())
                    .arg("--image-cache")
                    .arg(image_cache.to_string())
                    .output();
                match res {
                    Ok(output) => {
                        let preview = FilePreview::from(output);
                        let _ = event_tx.send(AppEvent::PreviewFile(path, Ok(preview)));
                    }
                    Err(e) => {
                        let _ = event_tx.send(AppEvent::PreviewFile(path, Err(e)));
                    }
                }
            });
        }
    }
}
