use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use crate::error::AppResult;
use crate::run::process_io::process_io_tasks;
use crate::types::event::AppEvent;
use crate::types::io::{IoTask, IoTaskStat};

/// Queues and tracks background file-operation ([`IoTask`]) execution on a dedicated worker
/// thread, one task at a time.
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
    pub progress: Option<IoTaskStat>,
}

impl WorkerState {
    /// Spawns the background IO worker thread and returns the queue that feeds it.
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
    /// Queues `thread` for execution and notifies the main loop a new task is pending.
    pub fn push_task(&mut self, thread: IoTask) {
        self.task_queue.push_back(thread);
        // error is ignored
        let _ = self.event_tx.send(AppEvent::NewIoTask);
    }
    /// Returns `true` if a task is currently executing.
    pub fn is_busy(&self) -> bool {
        self.progress.is_some()
    }
    /// Returns `true` if no tasks are queued.
    pub fn is_empty(&self) -> bool {
        self.task_queue.is_empty()
    }

    /// Returns an iterator over the queued (not-yet-started) tasks.
    pub fn iter<'a>(&'a self) -> Iter<'a, IoTask> {
        self.task_queue.iter()
    }
    /// Returns the progress of the currently executing task, if any.
    pub fn worker_ref(&self) -> Option<&IoTaskStat> {
        self.progress.as_ref()
    }

    /// Returns the current task's display-ready progress message, if any.
    pub fn get_msg(&self) -> Option<&str> {
        let worker = self.progress.as_ref()?;
        Some(worker.get_msg())
    }

    /// Dequeues and sends the next task to the worker thread, if any is queued.
    pub fn start_next_job(&mut self) -> AppResult {
        if let Some(task) = self.task_queue.pop_front() {
            let _ = self.task_tx.send(task);
        }
        Ok(())
    }

    /// Takes and clears the current task's progress state.
    pub fn remove_io_stat(&mut self) -> Option<IoTaskStat> {
        self.progress.take()
    }
}
