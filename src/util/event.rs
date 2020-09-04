use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

use crate::io::IOWorkerProgress;

#[derive(Debug)]
pub enum Event {
    Input(Key),
    IOWorkerProgress(IOWorkerProgress),
    IOWorkerResult(io::Result<IOWorkerProgress>),
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
    pub event_tx: mpsc::Sender<Event>,
    event_rx: mpsc::Receiver<Event>,
    pub input_tx: mpsc::SyncSender<()>,
    // fileio_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Self {
        Events::with_config()
    }

    pub fn with_config() -> Self {
        let (input_tx, input_rx) = mpsc::sync_channel(1);
        let (event_tx, event_rx) = mpsc::channel();

        {
            let event_tx = event_tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                let mut keys = stdin.keys();
                match keys.next() {
                    Some(key) => match key {
                        Ok(key) => {
                            if let Err(e) = event_tx.send(Event::Input(key)) {
                                eprintln!("Input thread send err: {:#?}", e);
                                return;
                            }
                        }
                        _ => return,
                    },
                    _ => return,
                }

                while let Ok(_) = input_rx.recv() {
                    if let Some(key) = keys.next() {
                        if let Ok(key) = key {
                            if let Err(e) = event_tx.send(Event::Input(key)) {
                                eprintln!("Input thread send err: {:#?}", e);
                                return;
                            }
                        }
                    }
                }
            })
        };

        Events {
            event_tx,
            event_rx,
            input_tx,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        let event = self.event_rx.recv()?;
        Ok(event)
    }

    pub fn flush(&self) {
        let _ = self.input_tx.send(());
    }
}
