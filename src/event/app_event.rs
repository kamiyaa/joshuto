use std::io;
use std::path;
use std::sync::mpsc;
use std::thread;

use signal_hook::consts::signal;
use signal_hook::iterator::exfiltrator::SignalOnly;
use signal_hook::iterator::SignalsInfo;

use termion::event::Event;
use termion::input::TermRead;

use uuid::Uuid;

use crate::error::JoshutoResult;
use crate::fs::JoshutoDirList;
use crate::io::FileOperationProgress;
use crate::preview::preview_file::FilePreview;

#[derive(Debug)]
pub enum AppEvent {
    // User input events
    Termion(Event),

    // background IO worker events
    IoWorkerCreate,
    FileOperationProgress(FileOperationProgress),
    IoWorkerResult(JoshutoResult<FileOperationProgress>),

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
        res: Box<io::Result<FilePreview>>,
    },
    // terminal size change events
    Signal(i32),
    // filesystem change events
    Filesystem(notify::Event),
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Config {}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    pub event_tx: mpsc::Sender<AppEvent>,
    event_rx: mpsc::Receiver<AppEvent>,
    pub input_tx: mpsc::SyncSender<()>,
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
        let _ = self.input_tx.send(());
    }
}

impl std::default::Default for Events {
    fn default() -> Self {
        let (input_tx, input_rx) = mpsc::sync_channel(1);
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

            while input_rx.recv().is_ok() {
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
