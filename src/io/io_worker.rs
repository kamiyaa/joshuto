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
            while let Ok(copied) = worker.recv() {
                let _ = event_tx.send(Event::IOWorkerProgress(copied));
            }
            let res = worker.join();
            let _ = event_tx.send(Event::IOWorkerResult(res));
        });

        Self { src, dest, handle }
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
    pub rx: mpsc::Receiver<u64>,
}

impl IOWorkerThread {
    pub fn start(&self) {
        self.tx_start.send(());
    }

    pub fn recv(&self) -> Result<u64, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn join(self) -> std::io::Result<u64> {
        match self.handle.join() {
            Ok(s) => s,
            Err(_) => Ok(0),
        }
    }
}
