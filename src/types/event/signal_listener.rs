use signal_hook::consts::signal;
use signal_hook::iterator::exfiltrator::SignalOnly;
use signal_hook::iterator::SignalsInfo;

use crate::types::event::{AppEvent, AppEventSender};

#[derive(Clone, Debug)]
pub struct SignalListener {
    pub event_tx: AppEventSender,
}

impl SignalListener {
    pub fn new(event_tx: AppEventSender) -> Self {
        Self { event_tx }
    }

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
