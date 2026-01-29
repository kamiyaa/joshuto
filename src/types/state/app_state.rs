use std::collections::HashSet;
use std::sync::mpsc;

use allmytoes::{AMTConfiguration, AMT};
use ratatui::style::Color;
use ratatui_image::picker::Picker;

use crate::commands::quit::QuitAction;
use crate::config::app::AppConfig;
use crate::types::event::{AppEvent, AppEventListener};
use crate::types::option::preview::PreviewProtocol;
use crate::types::state::{
    CommandLineState, MessageQueue, PreviewState, TabState, UiState, WorkerState,
};

use crate::{Args, THEME_T};

use super::{FileManagerState, ThreadPool};

pub struct AppState {
    pub config: AppConfig,
    pub quit: QuitAction,
    // event loop querying
    pub events: AppEventListener,
    // args from the command line
    pub args: Args,
    pub state: FileManagerState,
}

impl AppState {
    pub fn new(config: AppConfig, args: Args) -> Self {
        let picker = if config.preview_options.preview_shown_hook_script.is_none() {
            Picker::from_termios().ok().and_then(|mut picker| {
                picker.background_color = match THEME_T.preview_background {
                    Color::Rgb(r, g, b) => Some(image::Rgb([r, g, b])),
                    _ => None,
                };
                match config.preview_options.preview_protocol {
                    PreviewProtocol::Auto => {
                        picker.guess_protocol(); // Must run before Events::new() because it makes ioctl calls.
                        Some(picker)
                    }
                    PreviewProtocol::Disabled => None,
                    PreviewProtocol::ProtocolType(protocol_type) => {
                        picker.protocol_type = protocol_type;
                        Some(picker)
                    }
                }
            })
        } else {
            None
        };

        let events = AppEventListener::new();
        let event_tx = events.event_tx.clone();

        let commandline_state = CommandLineState::new();

        let event_tx_for_fs_notification = event_tx.clone();
        let watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                let _ = event_tx_for_fs_notification.send(AppEvent::Filesystem(event));
            }
        })
        .unwrap();
        let watched_paths = HashSet::with_capacity(3);

        let preview_script = config.preview_options.preview_script.clone();
        let allmytoes = if config.preview_options.use_xdg_thumbs {
            Some(AMT::new(&AMTConfiguration::default()))
        } else {
            None
        };
        let xdg_thumb_size = config.preview_options.xdg_thumb_size;

        Self {
            config,
            quit: QuitAction::DoNot,
            events,
            args,
            state: FileManagerState {
                tab_state: TabState::new(),
                local_state: None,
                search_state: None,
                message_queue: MessageQueue::new(),
                worker_state: WorkerState::new(event_tx.clone()),
                thread_pool: ThreadPool::new(),
                preview_state: PreviewState::new(
                    picker,
                    preview_script,
                    allmytoes,
                    xdg_thumb_size,
                    event_tx,
                ),
                ui_state: UiState { layout: vec![] },
                commandline_state,
                watcher,
                watched_paths,
                last_stdout: None,
            },
        }
    }

    // event related
    pub fn poll_event(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.events.next()
    }
    pub fn flush_event(&self) {
        self.events.flush();
    }
    pub fn clone_event_tx(&self) -> mpsc::Sender<AppEvent> {
        self.events.event_tx.clone()
    }
}
