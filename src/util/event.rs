use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

#[derive(Debug)]
pub enum Event {
    Input(Key),
    IOWorkerProgress(u64),
    IOWorkerResult,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {}

impl Default for Config {
    fn default() -> Config {
        Config {}
    }
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    prefix: &'static str,
    pub event_tx: mpsc::Sender<Event>,
    event_rx: mpsc::Receiver<Event>,
    pub sync_tx: mpsc::SyncSender<()>,
    sync_rx: mpsc::Receiver<()>,
    input_handle: thread::JoinHandle<()>,
    // fileio_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Self {
        Events::with_config("")
    }
    pub fn with_debug(s: &'static str) -> Self {
        let event = Events::with_config(s);
        event
    }

    pub fn with_config(prefix: &'static str) -> Self {
        let (sync_tx, sync_rx) = mpsc::sync_channel(1);
        let (event_tx, event_rx) = mpsc::channel();

        let input_handle = {
            let sync_tx = sync_tx.clone();
            let event_tx = event_tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                let mut keys = stdin.keys();
                while let Some(evt) = keys.next() {
                    match evt {
                        Ok(key) => {
                            if let Err(e) = event_tx.send(Event::Input(key)) {
                                eprintln!("[{}] Input thread send err: {:#?}", prefix, e);
                                return;
                            }
                            if let Err(_) = sync_tx.send(()) {
                                return;
                            }
                        }
                        _ => {}
                    }
                }
            })
        };

        Events {
            event_tx,
            event_rx,
            sync_tx,
            sync_rx,
            input_handle,
            prefix,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        let event = self.event_rx.recv()?;
        self.sync_rx.recv()?;
        Ok(event)
    }
    /*
        pub fn flush(&self) {
            self.sync_rx.try_recv();
        }
    */
}
