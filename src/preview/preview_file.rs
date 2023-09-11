use std::path;
use std::process::{Command, Output};
use std::sync::Mutex;
use std::thread;
use std::time;

use ratatui::layout::Rect;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::lazy_static;
use crate::ui::{views, AppBackend};

lazy_static! {
    static ref GUARD: Mutex<()> = Mutex::new(());
}

pub enum PreviewFileState {
    Loading,
    Error { message: String },
    Success { data: FilePreview },
}

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
    pub fn preview_path_with_script(
        context: &mut AppContext,
        backend: &mut AppBackend,
        path: path::PathBuf,
    ) {
        let preview_options = context.config_ref().preview_options_ref();
        if let Some(script) = preview_options.preview_script.as_ref() {
            if let Ok(area) = backend.terminal_ref().size() {
                let area = Rect {
                    y: area.top() + 1,
                    height: area.height - 2,
                    ..area
                };

                let config = context.config_ref();
                let display_options = config.display_options_ref();
                let constraints = &display_options.default_layout;
                let layout = if display_options.show_borders() {
                    views::calculate_layout_with_borders(area, constraints)
                } else {
                    views::calculate_layout(area, constraints)
                };
                let layout_rect = layout[2];
                let preview_width = layout_rect.width;
                let preview_height = layout_rect.height;

                if preview_width == 0 || preview_height == 0 {
                    return;
                }

                let script = script.clone();
                let event_tx = context.clone_event_tx();
                context
                    .preview_context_mut()
                    .previews_mut()
                    .insert(path.clone(), PreviewFileState::Loading);

                let _ = thread::spawn(move || {
                    let _locked = GUARD.lock().unwrap();
                    let file_full_path = path.as_path();

                    let res = Command::new(script)
                        .arg("--path")
                        .arg(file_full_path)
                        .arg("--preview-width")
                        .arg(preview_width.to_string())
                        .arg("--preview-height")
                        .arg(preview_height.to_string())
                        .output();
                    match res {
                        Ok(output) => {
                            let preview = FilePreview::from(output);
                            let res = AppEvent::PreviewFile {
                                path,
                                res: Box::new(Ok(preview)),
                            };
                            let _ = event_tx.send(res);
                        }
                        Err(e) => {
                            let res = AppEvent::PreviewFile {
                                path,
                                res: Box::new(Err(e)),
                            };
                            let _ = event_tx.send(res);
                        }
                    }
                });
            }
        }
    }
}
