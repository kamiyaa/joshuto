use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use crate::error::AppResult;
use crate::run::process_io::process_io_tasks;
use crate::types::event::AppEvent;
use crate::types::io::{FileOperationProgress, IoTask, IoTaskProgress};
use crate::utils::fs::query_number_of_items;

pub struct WorkerState {
    // to send info
    pub event_tx: mpsc::Sender<AppEvent>,
    // queue of IO workers
    pub task_queue: VecDeque<IoTask>,
    // communicate with worker thread
    pub task_tx: mpsc::Sender<IoTask>,
    // worker thread
    pub _handle: JoinHandle<()>,
    // current worker
    pub progress: Option<IoTaskProgress>,
}

impl WorkerState {
    pub fn new(event_tx: mpsc::Sender<AppEvent>) -> Self {
        let (task_tx, task_rx) = mpsc::channel();

        let event_tx_clone = event_tx.clone();
        let handle = thread::spawn(move || {
            let _ = process_io_tasks(task_rx, event_tx_clone);
        });

        Self {
            _handle: handle,
            event_tx,
            task_tx,
            task_queue: VecDeque::new(),
            progress: None,
        }
    }
    // worker related
    pub fn push_task(&mut self, thread: IoTask) {
        self.task_queue.push_back(thread);
        // error is ignored
        let _ = self.event_tx.send(AppEvent::IoTaskStart);
    }
    pub fn is_busy(&self) -> bool {
        self.progress.is_some()
    }
    pub fn is_empty(&self) -> bool {
        self.task_queue.is_empty()
    }

    pub fn iter(&self) -> Iter<IoTask> {
        self.task_queue.iter()
    }
    pub fn worker_ref(&self) -> Option<&IoTaskProgress> {
        self.progress.as_ref()
    }

    pub fn get_msg(&self) -> Option<&str> {
        let worker = self.progress.as_ref()?;
        Some(worker.get_msg())
    }

    pub fn start_next_job(&mut self) -> AppResult<()> {
        if let Some(task) = self.task_queue.pop_front() {
            let (total_files, total_bytes) = query_number_of_items(task.paths.as_slice())?;

            let src = task.paths[0].parent().unwrap().to_path_buf();
            let dest = task.dest.clone();

            let operation_progress = FileOperationProgress {
                kind: task.operation,
                current_file: task.paths[0].clone(),
                total_files,
                files_processed: 0,
                total_bytes,
                bytes_processed: 0,
            };

            let progress = IoTaskProgress::new(operation_progress, src, dest);
            self.progress = Some(progress);

            let _ = self.task_tx.send(task);
        }
        Ok(())
    }

    pub fn remove_worker(&mut self) -> Option<IoTaskProgress> {
        self.progress.take()
    }
}
