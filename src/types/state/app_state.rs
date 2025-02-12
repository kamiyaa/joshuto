use std::collections::HashSet;
use std::sync::mpsc;

use crate::commands::quit::QuitAction;
use crate::config::app::AppConfig;
use crate::types::event::{AppEvent, Events};
use crate::types::state::{
    CommandLineState, MessageQueue, PreviewState, TabState, UiState, WorkerState,
};

use crate::workers::thread_pool::ThreadPool;
use crate::Args;

use super::FileManagerState;

pub struct AppState {
    pub config: AppConfig,
    pub quit: QuitAction,
    // event loop querying
    pub events: Events,
    // args from the command line
    pub args: Args,
    pub state: FileManagerState,
}

impl AppState {
    pub fn new(config: AppConfig, args: Args) -> Self {
        let events = Events::new();
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

        let preview_state = PreviewState::new(&config.preview_options, event_tx.clone());
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
                preview_state,
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
