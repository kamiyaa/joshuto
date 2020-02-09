use crate::util::event::Event;
use std::sync::mpsc;
use std::thread;

#[derive(Clone, Debug)]
pub struct Options {
    pub overwrite: bool,
    pub skip_exist: bool,
}

impl std::default::Default for Options {
    fn default() -> Self {
        Self {
            overwrite: false,
            skip_exist: false,
        }
    }
}

pub struct IOWorkerThread {
    pub handle: thread::JoinHandle<std::io::Result<u64>>,
    pub tx_start: mpsc::Sender<()>,
    pub rx: mpsc::Receiver<Event>,
}

impl IOWorkerThread {
    pub fn start(&self) {
        self.tx_start.send(());
    }

    pub fn recv(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
