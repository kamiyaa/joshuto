use std::collections::HashMap;
use std::error::Error;
use std::path::{self, PathBuf};
use std::process::{self, Command, Stdio};
use std::sync::mpsc::{self, Sender};
use std::sync::Mutex;
use std::{io, thread};

use allmytoes::{ThumbSize, AMT};
use ratatui::layout::Rect;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::Protocol;
use ratatui_image::Resize;

use crate::config::clean::app::preview::PreviewOption;
use crate::config::clean::app::AppConfig;
use crate::event::{AppEvent, PreviewData};
use crate::lazy_static;
use crate::preview::preview_file::{FilePreview, PreviewFileState};
use crate::ui::{views, AppBackend, PreviewArea};
use crate::AppContext;

use super::{TabContext, UiContext};

lazy_static! {
    static ref GUARD: Mutex<()> = Mutex::new(());
}

type FilePreviewMetadata = HashMap<path::PathBuf, PreviewFileState>;

pub struct PreviewContext {
    // the last preview area (or None if now preview shown) to check if a preview hook script needs
    // to be called
    pub preview_area: Option<PreviewArea>,
    // hashmap of cached previews
    previews: FilePreviewMetadata,
    image_preview: Option<(PathBuf, Box<dyn Protocol>)>,
    sender_script: Sender<(PathBuf, Rect)>,
    sender_image: Option<Sender<(PathBuf, Rect)>>,
    // for telling main thread when previews are ready
    event_ts: Sender<AppEvent>,
}

impl PreviewContext {
    pub fn new(
        picker: Option<Picker>,
        script: Option<PathBuf>,
        allmytoes: Option<AMT>,
        xdg_thumb_size: ThumbSize,
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
                    let thumb_path = if let Some(amt) = &allmytoes {
                        let thumb_result = amt.get(&path, xdg_thumb_size);
                        if let Ok(thumb) = thumb_result {
                            PathBuf::from(thumb.path)
                        } else {
                            path.clone()
                        }
                    } else {
                        path.clone()
                    };
                    let proto = image::io::Reader::open(thumb_path.as_path())
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
            preview_area: None,
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

    pub fn update_external_preview(&mut self, preview_area: Option<PreviewArea>) {
        self.preview_area = preview_area;
    }

    /// Updates the external preview to the current preview in Joshuto.
    ///
    /// The function checks if the current preview content is the same as the preview content which
    /// has been last communicated to an external preview logic with the preview hook scripts.
    /// If the preview content has changed, one of the hook scripts is called. Either the "preview
    /// shown hook", if a preview is shown in Joshuto, or the "preview removed hook", if Joshuto has
    /// changed from an entry with preview to an entry without a preview.
    ///
    /// This function shall be called each time a change of Joshuto's preview can be expected.
    /// (As of now, it's called in each cycle of the main loop.)

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

/// Calls the "preview removed hook script" if it's configured.
pub fn call_preview_removed_hook(preview_options: &PreviewOption) {
    let preview_removed_hook_script = preview_options.preview_removed_hook_script.as_ref();
    if let Some(hook_script) = preview_removed_hook_script {
        let hook_script = hook_script.to_path_buf();
        let _ = thread::spawn(|| {
            let _ = process::Command::new(hook_script).status();
        });
    }
}

pub fn calculate_external_preview(
    tab_context: &TabContext,
    preview_context: &PreviewContext,
    ui_context: &UiContext,
    preview_options: &PreviewOption,
) -> Option<PreviewArea> {
    let layout = &ui_context.layout;
    let preview_area = views::calculate_preview(tab_context, preview_context, layout[2]);
    match preview_area.as_ref() {
        Some(new_preview_area) => {
            let should_preview = if let Some(old) = &preview_context.preview_area {
                new_preview_area.file_preview_path != old.file_preview_path
                    || new_preview_area.preview_area != old.preview_area
            } else {
                true
            };
            if should_preview {
                call_preview_shown_hook(new_preview_area.clone(), preview_options)
            }
        }
        None => {
            if preview_context.preview_area.is_some() {
                call_preview_removed_hook(preview_options)
            }
        }
    }
    preview_area
}
/// Calls the "preview shown hook script" if it's configured.
///
/// This method takes the current preview area as argument to check for both, the path of the
/// currently previewed file and the geometry of the preview area.
fn call_preview_shown_hook(preview_area: PreviewArea, preview_options: &PreviewOption) {
    let preview_shown_hook_script = preview_options.preview_shown_hook_script.as_ref();
    if let Some(hook_script) = preview_shown_hook_script {
        let hook_script = hook_script.to_path_buf();
        let _ = thread::spawn(move || {
            let _ = process::Command::new(hook_script.as_path())
                .arg(preview_area.file_preview_path.as_path())
                .arg(preview_area.preview_area.x.to_string())
                .arg(preview_area.preview_area.y.to_string())
                .arg(preview_area.preview_area.width.to_string())
                .arg(preview_area.preview_area.height.to_string())
                .status();
        });
    }
}

/// Remove the external preview, if any is present.
///
/// If the last preview hook script called was the "preview shown hook", this function will
/// call the "preview removed hook" to remove any external preview.
/// Otherwise it won't do anything.
///
/// To restore the external preview, `update_external_preview` is called which will detect the
/// difference and call the "preview shown hook" again for the current preview (if any).
///
/// This function can be called if an external preview shall be temporarily removed, for example
/// when entering the help screen.
pub fn remove_external_preview(context: &mut AppContext) {
    if context.preview_context_mut().preview_area.take().is_some() {
        let preview_options = context.config_ref().preview_options_ref();
        call_preview_removed_hook(preview_options);
    }
}
