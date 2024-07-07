use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

use crate::error::{AppError, AppErrorKind, AppResult};
use crate::types::event::AppEvent;
use crate::utils::fs::query_number_of_items;
use crate::workers::io::{FileOperationProgress, IoWorkerObserver, IoWorkerThread};

pub struct WorkerState {
    // to send info
    pub event_tx: mpsc::Sender<AppEvent>,
    // queue of IO workers
    pub worker_queue: VecDeque<IoWorkerThread>,
    // current worker
    pub observer: Option<IoWorkerObserver>,
}

impl WorkerState {
    pub fn new(event_tx: mpsc::Sender<AppEvent>) -> Self {
        Self {
            event_tx,
            worker_queue: VecDeque::new(),
            observer: None,
        }
    }
    pub fn clone_event_tx(&self) -> mpsc::Sender<AppEvent> {
        self.event_tx.clone()
    }
    // worker related
    pub fn push_worker(&mut self, thread: IoWorkerThread) {
        self.worker_queue.push_back(thread);
        // error is ignored
        let _ = self.event_tx.send(AppEvent::IoWorkerCreate);
    }
    pub fn is_busy(&self) -> bool {
        self.observer.is_some()
    }
    pub fn is_empty(&self) -> bool {
        self.worker_queue.is_empty()
    }

    pub fn iter(&self) -> Iter<IoWorkerThread> {
        self.worker_queue.iter()
    }
    pub fn worker_ref(&self) -> Option<&IoWorkerObserver> {
        self.observer.as_ref()
    }

    pub fn get_msg(&self) -> Option<&str> {
        let worker = self.observer.as_ref()?;
        Some(worker.get_msg())
    }

    pub fn start_next_job(&mut self) -> AppResult<()> {
        let tx = self.clone_event_tx();

        if let Some(worker) = self.worker_queue.pop_front() {
            let src = worker.paths[0].parent().unwrap().to_path_buf();
            let dest = worker.dest.clone();
            let (total_files, total_bytes) = query_number_of_items(worker.paths.as_slice())?;

            let operation_progress = FileOperationProgress {
                kind: worker.operation,
                current_file: worker.paths[0].clone(),
                total_files,
                files_processed: 0,
                total_bytes,
                bytes_processed: 0,
            };

            let handle = thread::spawn(move || {
                let (wtx, wrx) = mpsc::channel();
                // start worker
                let worker_handle = thread::spawn(move || worker.start(wtx));
                // relay worker info to event loop
                while let Ok(progress) = wrx.recv() {
                    let _ = tx.send(AppEvent::FileOperationProgress(progress));
                }
                let result = worker_handle.join();

                match result {
                    Ok(res) => {
                        let _ = tx.send(AppEvent::IoWorkerResult(res));
                    }
                    Err(_) => {
                        let err =
                            AppError::new(AppErrorKind::UnknownError, "Sending Error".to_string());
                        let _ = tx.send(AppEvent::IoWorkerResult(Err(err)));
                    }
                }
            });
            let observer = IoWorkerObserver::new(handle, operation_progress, src, dest);
            self.observer = Some(observer);
        }
        Ok(())
    }

    pub fn remove_worker(&mut self) -> Option<IoWorkerObserver> {
        self.observer.take()
    }
}
