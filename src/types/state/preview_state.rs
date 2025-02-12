use std::collections::HashMap;
use std::error::Error;
use std::path::{self, PathBuf};
use std::process;
use std::sync::mpsc::{self, Sender};
use std::sync::Mutex;
use std::{io, thread};

use allmytoes::{AMTConfiguration, AMT};
use lazy_static::lazy_static;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::Protocol;

use crate::config::app::AppConfig;
use crate::error::AppResult;
use crate::preview::preview_file::PreviewFileState;
use crate::types::event::AppEvent;
use crate::types::option::preview::{PreviewOption, PreviewProtocol};
use crate::ui::{views, AppBackend, PreviewArea};
use crate::workers::preview_image_worker::PreviewImageWorker;
use crate::workers::preview_worker::{PreviewWorker, PreviewWorkerRequest};
use crate::{AppState, THEME_T};

use super::{TabState, UiState};

lazy_static! {
    static ref GUARD: Mutex<()> = Mutex::new(());
}

type FilePreviewMap = HashMap<path::PathBuf, PreviewFileState>;

pub struct PreviewState {
    pub previews_store: FilePreviewMap,
    // the last preview area (or None if no preview shown) to check if a preview hook script needs
    // to be called
    pub preview_area: Option<PreviewArea>,
    // hashmap of cached previews
    pub image_preview: Option<(PathBuf, Box<Protocol>)>,
    pub preview_file_req_tx: Sender<PreviewWorkerRequest>,
    pub preview_image_req_tx: Option<Sender<PreviewWorkerRequest>>,
    // for telling main thread when previews are ready
    pub event_tx: Sender<AppEvent>,
}

impl PreviewState {
    pub fn new(options: &PreviewOption, event_tx: Sender<AppEvent>) -> PreviewState {
        // preview worker
        let preview_script = options.preview_script.clone();
        let (preview_file_req_tx, preview_file_req_rx) = mpsc::channel();
        if let Some(preview_script) = preview_script {
            let preview_worker = PreviewWorker {
                preview_script,
                response_tx: event_tx.clone(),
                request_rx: preview_file_req_rx,
            };
            thread::spawn(move || preview_worker.listen_for_events());
        }

        // preview image worker
        let preview_image_req_tx = if options.preview_shown_hook_script.is_none() {
            let picker = Picker::from_query_stdio().ok().and_then(|mut picker| {
                if let Color::Rgb(r, g, b) = THEME_T.preview_background {
                    picker.set_background_color([255, r, g, b]);
                }
                match options.preview_protocol {
                    PreviewProtocol::Disabled => None,
                    PreviewProtocol::ProtocolType(protocol_type) => {
                        picker.set_protocol_type(protocol_type);
                        Some(picker)
                    }
                }
            });
            let allmytoes = if options.use_xdg_thumbs {
                Some(AMT::new(&AMTConfiguration::default()))
            } else {
                None
            };
            let xdg_thumb_size = options.xdg_thumb_size;

            let (preview_image_req_tx, preview_image_req_rx) =
                mpsc::channel::<PreviewWorkerRequest>();
            let preview_image_worker = PreviewImageWorker {
                response_tx: event_tx.clone(),
                request_rx: preview_image_req_rx,
                picker,
                allmytoes,
                xdg_thumb_size,
            };
            thread::spawn(move || preview_image_worker.listen_for_events());
            Some(preview_image_req_tx)
        } else {
            None
        };

        PreviewState {
            preview_area: None,
            previews_store: HashMap::new(),
            image_preview: None,
            preview_file_req_tx,
            preview_image_req_tx,
            event_tx,
        }
    }

    pub fn load_preview_lazy(
        &mut self,
        config: &AppConfig,
        backend: &AppBackend,
        path: path::PathBuf,
    ) -> AppResult {
        // always load image without cache
        self.set_image_preview(None);
        self.load_preview_image(config, backend, path.clone())?;

        let previews = self.previews_mut();
        if previews.get(path.as_path()).is_none() {
            // add to loading state
            previews.insert(path.clone(), PreviewFileState::Loading);
            self.load_preview_script(config, backend, path)?;
        }
        Ok(())
    }

    pub fn previews_ref(&self) -> &FilePreviewMap {
        &self.previews_store
    }
    pub fn previews_mut(&mut self) -> &mut FilePreviewMap {
        &mut self.previews_store
    }
    pub fn image_preview_ref(&self, other: &path::Path) -> Option<&Protocol> {
        match &self.image_preview {
            Some((path, protocol)) if path == other => Some(protocol.as_ref()),
            _ => None,
        }
    }
    pub fn set_image_preview(&mut self, preview: Option<(path::PathBuf, Box<Protocol>)>) {
        self.image_preview = preview;
    }

    pub fn load_preview_script(
        &self,
        config: &AppConfig,
        backend: &AppBackend,
        path: path::PathBuf,
    ) -> AppResult {
        let rect = Self::backend_rect(config, backend)?;
        let res = self
            .preview_file_req_tx
            .send(PreviewWorkerRequest {
                path: path.clone(),
                area: rect,
            })
            .map_err(Self::map_io_err);
        if let Err(err) = res {
            let app_event = AppEvent::PreviewFile {
                path,
                res: Err(err),
            };
            let _ = self.event_tx.send(app_event);
        }
        Ok(())
    }

    pub fn load_preview_image(
        &self,
        config: &AppConfig,
        backend: &AppBackend,
        path: path::PathBuf,
    ) -> AppResult {
        if let Some(sender) = &self.preview_image_req_tx {
            let area = Self::backend_rect(config, backend)?;
            let res = sender
                .send(PreviewWorkerRequest {
                    path: path.clone(),
                    area,
                })
                .map_err(Self::map_io_err);
            if let Err(err) = res {
                let app_event = AppEvent::PreviewFile {
                    path,
                    res: Err(err),
                };
                let _ = self.event_tx.send(app_event);
            }
        }
        Ok(())
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
        let size = backend.terminal_ref().size()?;
        let area = Rect {
            x: 0,
            y: 1,
            height: size.height - 2,
            width: size.width,
        };

        let display_options = &config.display_options;
        let constraints = &display_options.default_layout;
        let layout = if display_options.show_borders {
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
    tab_state: &TabState,
    preview_state: &PreviewState,
    ui_state: &UiState,
    preview_options: &PreviewOption,
) -> Option<PreviewArea> {
    let layout = &ui_state.layout;
    let preview_area = views::calculate_preview(tab_state, preview_state, layout[2]);
    match preview_area.as_ref() {
        Some(new_preview_area) => {
            let should_preview = if let Some(old) = &preview_state.preview_area {
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
            if preview_state.preview_area.is_some() {
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
pub fn remove_external_preview(app_state: &mut AppState) {
    if app_state
        .state
        .preview_state_mut()
        .preview_area
        .take()
        .is_some()
    {
        let preview_options = &app_state.config.preview_options;
        call_preview_removed_hook(preview_options);
    }
}
