use std::fmt::Debug;
use std::io;
use std::path;
use std::sync::mpsc;
use std::thread;

use ratatui_image::protocol::Protocol;
use signal_hook::consts::signal;
use signal_hook::iterator::exfiltrator::SignalOnly;
use signal_hook::iterator::SignalsInfo;

use termion::event::Event;
use termion::input::TermRead;

use uuid::Uuid;

use crate::error::AppResult;
use crate::fs::JoshutoDirList;
use crate::preview::preview_file::FilePreview;
use crate::types::io::IoTaskProgressMessage;
use crate::types::io::IoTaskStat;

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
    Termion(Event),

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
pub struct Events {
    pub event_tx: mpsc::Sender<AppEvent>,
    event_rx: mpsc::Receiver<AppEvent>,
    pub input_tx: mpsc::Sender<()>,
}

impl Events {
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

impl std::default::Default for Events {
    fn default() -> Self {
        let (input_tx, input_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        // edge case that starts off the input thread
        let _ = input_tx.send(());

        // signal thread
        let event_tx2 = event_tx.clone();
        let _ = thread::spawn(move || {
            let sigs = vec![signal::SIGWINCH];
            let mut signals = SignalsInfo::<SignalOnly>::new(sigs).unwrap();
            for signal in &mut signals {
                if let Err(e) = event_tx2.send(AppEvent::Signal(signal)) {
                    eprintln!("Signal thread send err: {:#?}", e);
                    return;
                }
            }
        });

        // input thread
        let event_tx2 = event_tx.clone();
        let _ = thread::spawn(move || {
            let stdin = io::stdin();
            let mut events = stdin.events();

            loop {
                let _ = input_rx.recv();
                if let Some(Ok(event)) = events.next() {
                    let _ = event_tx2.send(AppEvent::Termion(event));
                }
            }
        });

        Events {
            event_tx,
            event_rx,
            input_tx,
        }
    }
}
