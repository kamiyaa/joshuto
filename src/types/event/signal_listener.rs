use signal_hook::consts::signal;
use signal_hook::iterator::exfiltrator::SignalOnly;
use signal_hook::iterator::SignalsInfo;

use crate::types::event::{AppEvent, AppEventSender};

/// Listens for OS signals (currently `SIGWINCH`) and forwards them as [`AppEvent`]s.
#[derive(Clone, Debug)]
pub struct SignalListener {
    pub event_tx: AppEventSender,
}

impl SignalListener {
    /// Builds a listener that sends signal events to `event_tx`.
    pub fn new(event_tx: AppEventSender) -> Self {
        Self { event_tx }
    }

    /// Runs the listener loop, forwarding each received signal until the channel closes.
    /// Intended to be run on its own thread.
    pub fn run(self) {
        let sigs = vec![signal::SIGWINCH];
        let mut signals = SignalsInfo::<SignalOnly>::new(&sigs).unwrap();
        for signal in &mut signals {
            if let Err(e) = self.event_tx.send(AppEvent::Signal(signal)) {
                eprintln!("Signal thread send err: {:#?}", e);
                return;
            }
        }
    }
}
