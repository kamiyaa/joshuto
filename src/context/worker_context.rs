use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

use crate::event::AppEvent;
use crate::io::{IoWorkerObserver, IoWorkerProgress, IoWorkerThread};

pub struct WorkerContext {
    // to send info
    event_tx: mpsc::Sender<AppEvent>,
    // queue of IO workers
    worker_queue: VecDeque<IoWorkerThread>,
    // current worker
    worker: Option<IoWorkerObserver>,
}

impl WorkerContext {
    pub fn new(event_tx: mpsc::Sender<AppEvent>) -> Self {
        Self {
            event_tx,
            worker_queue: VecDeque::new(),
            worker: None,
        }
    }
    pub fn clone_event_tx(&self) -> mpsc::Sender<AppEvent> {
        self.event_tx.clone()
    }
    // worker related
    pub fn push(&mut self, thread: IoWorkerThread) {
        self.worker_queue.push_back(thread);
        // error is ignored
        let _ = self.event_tx.send(AppEvent::IoWorkerCreate);
    }
    pub fn is_busy(&self) -> bool {
        self.worker.is_some()
    }
    pub fn is_empty(&self) -> bool {
        self.worker_queue.is_empty()
    }

    pub fn iter(&self) -> Iter<IoWorkerThread> {
        self.worker_queue.iter()
    }
    pub fn worker_ref(&self) -> Option<&IoWorkerObserver> {
        self.worker.as_ref()
    }

    pub fn set_progress(&mut self, res: IoWorkerProgress) {
        if let Some(s) = self.worker.as_mut() {
            s.set_progress(res);
        }
    }

    pub fn get_msg(&self) -> Option<&str> {
        let worker = self.worker.as_ref()?;
        Some(worker.get_msg())
    }
    pub fn update_msg(&mut self) {
        if let Some(s) = self.worker.as_mut() {
            s.update_msg();
        }
    }

    pub fn start_next_job(&mut self) {
        let tx = self.clone_event_tx();

        if let Some(worker) = self.worker_queue.pop_front() {
            let src = worker.paths[0].parent().unwrap().to_path_buf();
            let dest = worker.dest.clone();
            let handle = thread::spawn(move || {
                let (wtx, wrx) = mpsc::channel();
                // start worker
                let worker_handle = thread::spawn(move || worker.start(wtx));
                // relay worker info to event loop
                while let Ok(progress) = wrx.recv() {
                    let _ = tx.send(AppEvent::IoWorkerProgress(progress));
                }
                let result = worker_handle.join();

                match result {
                    Ok(res) => {
                        let _ = tx.send(AppEvent::IoWorkerResult(res));
                    }
                    Err(_) => {
                        let err = std::io::Error::new(std::io::ErrorKind::Other, "Sending Error");
                        let _ = tx.send(AppEvent::IoWorkerResult(Err(err)));
                    }
                }
            });
            let observer = IoWorkerObserver::new(handle, src, dest);
            self.worker = Some(observer);
        }
    }

    pub fn remove_worker(&mut self) -> Option<IoWorkerObserver> {
        self.worker.take()
    }
}
