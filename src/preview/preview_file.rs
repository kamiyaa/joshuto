use std::io;
use std::path;
use std::process::{Command, Output};
use std::thread;

use tui::layout::Constraint;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::ui::{self, TuiBackend};

#[derive(Clone, Debug)]
pub struct FilePreview {
    pub _path: path::PathBuf,
    pub status: std::process::ExitStatus,
    pub output: String,
}

impl std::convert::From<(path::PathBuf, Output)> for FilePreview {
    fn from((p, output): (path::PathBuf, Output)) -> Self {
        let s = String::from_utf8_lossy(&output.stdout).to_string();
        let status = output.status;
        Self {
            _path: p,
            status,
            output: s,
        }
    }
}

pub struct Foreground {}

impl Foreground {
    pub fn preview_path_with_script(
        context: &AppContext,
        backend: &mut TuiBackend,
        p: path::PathBuf,
    ) -> io::Result<Output> {
        let preview_options = context.config_ref().preview_options_ref();
        let config = context.config_ref();

        match preview_options.preview_script.as_ref() {
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                "No preview script specified",
            )),
            Some(script) => {
                let area = backend.terminal.as_ref().unwrap().size().unwrap();
                let display_options = config.display_options_ref();
                let constraints: &[Constraint; 3] = &display_options.default_layout;

                let layout_rect = ui::build_layout(area, constraints, display_options)[2];

                let file_full_path = p.as_path();
                let preview_width = layout_rect.width;
                let preview_height = layout_rect.height;
                let image_cache = 0;
                let preview_image = if preview_options.preview_images { 1 } else { 0 };

                // spawn preview process
                Command::new(script)
                    .arg(file_full_path)
                    .arg(preview_width.to_string())
                    .arg(preview_height.to_string())
                    .arg(image_cache.to_string())
                    .arg(preview_image.to_string())
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
        p: path::PathBuf,
    ) {
        if context
            .preview_context_ref()
            .get_preview(p.as_path())
            .is_some()
        {
            return;
        }

        let preview_options = context.config_ref().preview_options_ref();
        let config = context.config_ref();

        if let Some(script) = preview_options.preview_script.as_ref() {
            let area = backend.terminal.as_ref().unwrap().size().unwrap();
            let display_options = config.display_options_ref();
            let constraints: &[Constraint; 3] = &display_options.default_layout;

            let layout_rect = ui::build_layout(area, constraints, display_options)[2];

            let preview_width = layout_rect.width;
            let preview_height = layout_rect.height;
            let image_cache = 0;
            let preview_image = if preview_options.preview_images { 1 } else { 0 };

            let script = script.clone();
            let event_tx = context.clone_event_tx();
            let _ = thread::spawn(move || {
                let file_full_path = p.as_path();

                let res = Command::new(script)
                    .arg(file_full_path)
                    .arg(preview_width.to_string())
                    .arg(preview_height.to_string())
                    .arg(image_cache.to_string())
                    .arg(preview_image.to_string())
                    .output();
                match res {
                    Ok(output) => {
                        let preview = FilePreview::from((p, output));
                        let _ = event_tx.send(AppEvent::PreviewFile(preview));
                    }
                    Err(_) => {}
                }
            });
        }
    }
}
