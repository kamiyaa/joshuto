use std::sync::mpsc;
use std::thread;

use crate::tasks::app_event::AppEventReceiver;
use crate::tasks::app_event::AppEventSender;
use crate::tasks::input_listener::TerminalInputListener;
use crate::tasks::signal_listener::SignalListener;
use crate::tasks::AppEvent;

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct AppEventListener {
    pub event_tx: AppEventSender,
    event_rx: AppEventReceiver,
    pub input_tx: mpsc::Sender<()>,
}

impl AppEventListener {
    /// Spawns the signal and terminal-input listener threads and returns the event channel.
    pub fn new() -> Self {
        Self::default()
    }

    // We need a next() and a flush() so we don't continuously consume
    // input from the console. Sometimes, other applications need to
    // read terminal inputs while joshuto is in the background
    /// Blocks until the next `AppEvent` arrives.
    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        let event = self.event_rx.recv()?;
        Ok(event)
    }

    /// Signals the input thread to poll for the next terminal input event.
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
