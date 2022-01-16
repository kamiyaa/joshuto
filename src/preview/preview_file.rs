use std::io;
use std::path;
use std::process::{Command, Output};
use std::thread;

use tui::layout::Constraint;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::ui::{self, TuiBackend};

#[derive(Clone, Debug)]
pub enum PreviewState {
    NoPreview,
    SomePreview(FilePreview),
}

#[derive(Clone, Debug)]
pub struct FilePreview {
    pub status: std::process::ExitStatus,
    pub output: String,
    pub index: usize,
}

impl std::convert::From<Output> for FilePreview {
    fn from(output: Output) -> Self {
        let s = String::from_utf8_lossy(&output.stdout).to_string();
        let s2 = s.replace('\t', "        ");
        let status = output.status;
        Self {
            status,
            output: s2,
            index: 0,
        }
    }
}

#[allow(dead_code)]
pub struct Foreground {}

impl Foreground {
    pub fn preview_path_with_script(
        context: &AppContext,
        backend: &mut TuiBackend,
        p: path::PathBuf,
    ) -> io::Result<Output> {
        let config = context.config_ref();
        let preview_options = config.preview_options_ref();

        match preview_options.preview_script.as_ref() {
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                "No preview script specified",
            )),
            Some(script) => {
                let area = backend.terminal.as_ref().unwrap().size().unwrap();
                let display_options = config.display_options_ref();
                let constraints: &[Constraint; 3] = &display_options.default_layout;

                let ui_context = context.ui_context_ref();
                if ui_context.layout.is_empty() {
                    return Err(io::Error::new(io::ErrorKind::Other, "No preview area"));
                }
                let layout_rect = &ui_context.layout[ui_context.layout.len() - 1];

                let file_full_path = p.as_path();
                let preview_width = layout_rect.width;
                let preview_height = layout_rect.height;
                let preview_x_coord = layout_rect.x;
                let preview_y_coord = layout_rect.y;

                let image_cache = 0;
                let preview_image = if preview_options.preview_images { 1 } else { 0 };

                // spawn preview process
                Command::new(script)
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
                    .arg("--preview-images")
                    .arg(preview_image.to_string())
                    .arg("--image-cache")
                    .arg(image_cache.to_string())
                    .output()
            }
        }
    }

    pub fn preview_with_script(
        context: &AppContext,
        backend: &mut TuiBackend,
    ) -> io::Result<Output> {
        let curr_tab = context.tab_context_ref().curr_tab_ref();
        let child_list = curr_tab.child_list_ref();

        match child_list.and_then(|list| list.curr_entry_ref()) {
            None => Err(io::Error::new(io::ErrorKind::Other, "No file to preview")),
            Some(entry) => {
                Self::preview_path_with_script(context, backend, entry.file_path().to_path_buf())
            }
        }
    }
}

pub struct Background {}

impl Background {
    pub fn preview_path_with_script(
        context: &AppContext,
        backend: &mut TuiBackend,
        path: path::PathBuf,
    ) {
        let preview_options = context.config_ref().preview_options_ref();
        let config = context.config_ref();

        if let Some(script) = preview_options.preview_script.as_ref() {
            let area = backend.terminal.as_ref().unwrap().size().unwrap();
            let display_options = config.display_options_ref();
            let constraints: &[Constraint; 3] = &display_options.default_layout;

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
            let preview_image = if preview_options.preview_images { 1 } else { 0 };

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
                    .arg("--preview-images")
                    .arg(preview_image.to_string())
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
