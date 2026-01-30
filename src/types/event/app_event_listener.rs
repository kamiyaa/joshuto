use std::fmt::Debug;
use std::io;
use std::path;
use std::sync::mpsc;
use std::thread;

use ratatui::termion::event::Event;
use ratatui_image::protocol::Protocol;

use uuid::Uuid;

use crate::error::AppResult;
use crate::fs::JoshutoDirList;
use crate::preview::preview_file::FilePreview;
use crate::types::event::input_listener::TerminalInputListener;
use crate::types::event::signal_listener::SignalListener;
use crate::types::io::IoTaskProgressMessage;
use crate::types::io::IoTaskStat;

pub type AppEventSender = mpsc::Sender<AppEvent>;
pub type AppEventReceiver = mpsc::Receiver<AppEvent>;

pub enum PreviewData {
    Script(Box<FilePreview>),
    Image(Box<dyn Protocol>),
}

impl Debug for PreviewData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Script(_) => f.debug_tuple("Script").field(&"_").finish(),
            Self::Image(_) => f.debug_tuple("Image").field(&"_").finish(),
        }
    }
}

#[derive(Debug)]
pub enum AppEvent {
    // User input events
    TerminalEvent(Event),

    // background IO worker events
    NewIoTask,
    IoTaskStart(IoTaskStat),
    IoTaskProgress(IoTaskProgressMessage),
    IoTaskResult(AppResult),

    // forked process events
    ChildProcessComplete(u32),

    // preview thread events
    PreviewDir {
        id: Uuid,
        path: path::PathBuf,
        res: Box<io::Result<JoshutoDirList>>,
    },
    PreviewFile {
        path: path::PathBuf,
        res: io::Result<PreviewData>,
    },
    // terminal size change events
    Signal(i32),
    // filesystem change events
    Filesystem(notify::Event),
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct AppEventListener {
    pub event_tx: AppEventSender,
    event_rx: AppEventReceiver,
    pub input_tx: mpsc::Sender<()>,
}

impl AppEventListener {
    pub fn new() -> Self {
        Self::default()
    }

    // We need a next() and a flush() so we don't continuously consume
    // input from the console. Sometimes, other applications need to
    // read terminal inputs while joshuto is in the background
    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        let event = self.event_rx.recv()?;
        Ok(event)
    }

    pub fn flush(&self) {
        loop {
            if self.input_tx.send(()).is_ok() {
                break;
            }
        }
    }
}

impl std::default::Default for AppEventListener {
    fn default() -> Self {
        let (input_tx, input_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        // signal thread
        let signal_listener = SignalListener::new(event_tx.clone());
        let _ = thread::spawn(move || {
            signal_listener.run();
        });

        // edge case that starts off the input thread
        let _ = input_tx.send(());
        // input thread
        let input_listener = TerminalInputListener::new(event_tx.clone(), input_rx);
        let _ = thread::spawn(move || {
            input_listener.run();
        });

        AppEventListener {
            event_tx,
            event_rx,
            input_tx,
        }
    }
}
