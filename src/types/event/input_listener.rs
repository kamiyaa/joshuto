use std::{io, sync::mpsc::Receiver};

use ratatui::termion::input::TermRead;

use crate::types::event::{AppEvent, AppEventSender};

/// Signals the terminal-input thread to poll for the next input event.
pub type InputEventReceiver = Receiver<()>;

/// Listens for terminal inputs
#[derive(Debug)]
pub struct TerminalInputListener {
    pub event_tx: AppEventSender,
    // Used to make sure we only poll for terminal input when we want to
    pub input_rx: InputEventReceiver,
}

impl TerminalInputListener {
    /// Builds a listener that sends terminal input events to `event_tx` on request.
    pub fn new(event_tx: AppEventSender, input_rx: InputEventReceiver) -> Self {
        Self { event_tx, input_rx }
    }

    /// Runs the listener loop: waits for a poll request, then reads and forwards one terminal
    /// input event. Intended to be run on its own thread.
    pub fn run(self) {
        let stdin = io::stdin();
        let mut events = stdin.events();

        loop {
            let _ = self.input_rx.recv();
            if let Some(Ok(event)) = events.next() {
                let _ = self.event_tx.send(AppEvent::TerminalEvent(event));
            }
        }
    }
}
