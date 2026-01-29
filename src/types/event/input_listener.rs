use std::{io, sync::mpsc::Receiver};

use ratatui::termion::input::TermRead;

use crate::types::event::{AppEvent, AppEventSender};

pub type InputEventReceiver = Receiver<()>;

/// Listens for terminal inputs
#[derive(Debug)]
pub struct TerminalInputListener {
    pub event_tx: AppEventSender,
    // Used to make sure we only poll for terminal input when we want to
    pub input_rx: InputEventReceiver,
}

impl TerminalInputListener {
    pub fn new(event_tx: AppEventSender, input_rx: InputEventReceiver) -> Self {
        Self { event_tx, input_rx }
    }

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
