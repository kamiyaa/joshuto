use std::path;
use std::sync::mpsc;
use std::thread;

use crate::util::event::Event;

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

pub struct IOWorkerObserver {
    pub src: path::PathBuf,
    pub dest: path::PathBuf,
    pub handle: std::thread::JoinHandle<()>,
}

impl IOWorkerObserver {
    pub fn new(worker: IOWorkerThread, event_tx: mpsc::Sender<Event>) -> Self {
        let src = worker.src.clone();
        let dest = worker.dest.clone();

        let handle = thread::spawn(move || {
            worker.start();
            while let Ok(evt) = worker.recv() {
                let _ = event_tx.send(evt);
            }
            worker.handle.join();
            let _ = event_tx.send(Event::IOWorkerResult);
        });

        Self {
            src,
            dest,
            handle,
        }
    }

    pub fn join(self) {
        self.handle.join();
    }
}

pub struct IOWorkerThread {
    pub src: path::PathBuf,
    pub dest: path::PathBuf,
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
