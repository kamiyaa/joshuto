use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime};

use chrono::offset::Local;
use chrono::DateTime;

use termion::event::Key;
use termion::input::TermRead;

use crate::io::IOWorkerThread;

#[derive(Debug)]
pub enum Event {
    Input(Key),
    IOWorkerProgress(u64),
    IOWorkerResult,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tick_rate: Duration::from_millis(250),
        }
    }
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    pub event_tx: mpsc::Sender<Event>,
    event_rx: mpsc::Receiver<Event>,
    pub sync_tx: mpsc::Sender<()>,
    sync_rx: mpsc::Receiver<()>,
    input_handle: thread::JoinHandle<()>,
    // fileio_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Self {
        Events::with_config(Config::default())
    }

    pub fn with_config(_: Config) -> Self {
        let (sync_tx, sync_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        let input_handle = {
            let sync_tx = sync_tx.clone();
            let event_tx = event_tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                let mut keys = stdin.keys();
                loop {
                    if let Some(evt) = keys.next() {
                        match evt {
                            Ok(key) => {
                                if let Err(e) = event_tx.send(Event::Input(key)) {
                                    eprintln!("Input thread send err: {:#?}", e);
                                    return;
                                }
                                sync_tx.send(());
                            }
                            _ => {}
                        }
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
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        let now = SystemTime::now();
        let datetime: DateTime<Local> = now.into();
        #[cfg(debug_assertions)]
        eprintln!(
            "\nwaiting for event...{}",
            datetime.format("%d/%m/%Y %T %6f")
        );

        let event = self.event_rx.recv();

        #[cfg(debug_assertions)]
        eprintln!("got event: {:?}", event);

        let now = SystemTime::now();
        let datetime: DateTime<Local> = now.into();

        #[cfg(debug_assertions)]
        eprintln!("Event captured at: {}", datetime.format("%d/%m/%Y %T %6f"));

        #[cfg(debug_assertions)]
        eprintln!("waiting for recv...");

        self.sync_rx.recv();
        let now = SystemTime::now();
        let datetime: DateTime<Local> = now.into();

        #[cfg(debug_assertions)]
        eprintln!("Done: {}", datetime.format("%d/%m/%Y %T %6f"));
        event
    }
}
