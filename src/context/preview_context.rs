use std::collections::HashMap;
use std::error::Error;
use std::path::{self, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Sender};
use std::sync::Mutex;
use std::{io, thread};

use ratatui::layout::Rect;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::Protocol;
use ratatui_image::Resize;

use crate::config::clean::app::AppConfig;
use crate::event::{AppEvent, PreviewData};
use crate::lazy_static;
use crate::preview::preview_file::{FilePreview, PreviewFileState};
use crate::ui::{views, AppBackend};
use crate::AppContext;

lazy_static! {
    static ref GUARD: Mutex<()> = Mutex::new(());
}

type FilePreviewMetadata = HashMap<path::PathBuf, PreviewFileState>;

pub struct PreviewContext {
    previews: FilePreviewMetadata,
    image_preview: Option<(PathBuf, Box<dyn Protocol>)>,
    sender_script: Sender<(PathBuf, Rect)>,
    sender_image: Option<Sender<(PathBuf, Rect)>>,
    event_ts: Sender<AppEvent>,
}

impl PreviewContext {
    pub fn new(
        picker: Option<Picker>,
        script: Option<PathBuf>,
        event_ts: Sender<AppEvent>,
    ) -> PreviewContext {
        let (sender_script, receiver) = mpsc::channel::<(PathBuf, Rect)>();
        let thread_script_event_ts = event_ts.clone();
        thread::spawn(move || {
            for (path, rect) in receiver {
                if let Some(ref script) = script {
                    PreviewContext::spawn_command(
                        path.clone(),
                        script.to_path_buf(),
                        rect,
                        thread_script_event_ts.clone(),
                    );
                }
            }
        });

        let (sender_image, receiver) = mpsc::channel::<(PathBuf, Rect)>();
        let sender_image = picker.map(|mut picker| {
            let thread_image_event_ts = event_ts.clone();
            thread::spawn(move || loop {
                // Get last, or block for next.
                if let Some((path, rect)) = receiver
                    .try_iter()
                    .last()
                    .or_else(|| receiver.iter().next())
                {
                    let proto = image::io::Reader::open(path.as_path())
                        .and_then(|reader| reader.decode().map_err(Self::map_io_err))
                        .and_then(|dyn_img| {
                            picker
                                .new_protocol(dyn_img, rect, Resize::Fit)
                                .map_err(|err| {
                                    io::Error::new(io::ErrorKind::Other, format!("{err}"))
                                })
                        });
                    if let Ok(proto) = proto {
                        let ev = AppEvent::PreviewFile {
                            path,
                            res: Ok(PreviewData::Image(proto)),
                        };
                        let _ = thread_image_event_ts.send(ev);
                    }
                } else {
                    // Closed.
                    return;
                }
            });
            sender_image
        });

        PreviewContext {
            previews: HashMap::new(),
            image_preview: None,
            sender_script,
            sender_image,
            event_ts,
        }
    }

    fn spawn_command(
        path: PathBuf,
        script: PathBuf,
        rect: Rect,
        thread_event_ts: Sender<AppEvent>,
    ) {
        let output = Command::new(script)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .arg("--path")
            .arg(path.as_path())
            .arg("--preview-width")
            .arg(rect.width.to_string())
            .arg("--preview-height")
            .arg(rect.height.to_string())
            .output();

        let res = match output {
            Ok(output) => {
                if output.status.success() {
                    let preview = FilePreview::from(output);
                    AppEvent::PreviewFile {
                        path,
                        res: Ok(PreviewData::Script(Box::new(preview))),
                    }
                } else {
                    AppEvent::PreviewFile {
                        path,
                        res: Err(io::Error::new(io::ErrorKind::Other, "nonzero status")),
                    }
                }
            }
            Err(err) => AppEvent::PreviewFile {
                path,
                res: Err(io::Error::new(io::ErrorKind::Other, format!("{err}"))),
            },
        };
        let _ = thread_event_ts.send(res);
    }

    pub fn previews_ref(&self) -> &FilePreviewMetadata {
        &self.previews
    }
    pub fn previews_mut(&mut self) -> &mut FilePreviewMetadata {
        &mut self.previews
    }
    pub fn image_preview_ref(&self, other: &path::Path) -> Option<&dyn Protocol> {
        match &self.image_preview {
            Some((path, protocol)) if path == other => Some(protocol.as_ref()),
            _ => None,
        }
    }
    pub fn set_image_preview(&mut self, preview: Option<(path::PathBuf, Box<dyn Protocol>)>) {
        self.image_preview = preview;
    }

    pub fn load_preview_script(
        &self,
        context: &AppContext,
        backend: &AppBackend,
        path: path::PathBuf,
    ) {
        if let Err(err) = Self::backend_rect(context.config_ref(), backend).and_then(|rect| {
            self.sender_script
                .send((path.clone(), rect))
                .map_err(Self::map_io_err)
        }) {
            let ev = AppEvent::PreviewFile {
                path,
                res: Err(err),
            };
            let _ = self.event_ts.send(ev);
        }
    }

    pub fn load_preview_image(
        &self,
        context: &AppContext,
        backend: &AppBackend,
        path: path::PathBuf,
    ) {
        if let Some(sender) = &self.sender_image {
            if let Err(err) = Self::backend_rect(context.config_ref(), backend)
                .and_then(|rect| sender.send((path.clone(), rect)).map_err(Self::map_io_err))
            {
                let ev = AppEvent::PreviewFile {
                    path,
                    res: Err(err),
                };
                let _ = self.event_ts.send(ev);
            }
        }
    }

    fn backend_rect(config: &AppConfig, backend: &AppBackend) -> io::Result<Rect> {
        let area = backend.terminal_ref().size()?;
        let area = Rect {
            y: area.top() + 1,
            height: area.height - 2,
            ..area
        };

        let display_options = config.display_options_ref();
        let constraints = &display_options.default_layout;
        let layout = if display_options.show_borders() {
            views::calculate_layout_with_borders(area, constraints)
        } else {
            views::calculate_layout(area, constraints)
        };
        Ok(layout[2])
    }

    #[inline]
    fn map_io_err(err: impl Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, format!("{err}"))
    }
}
