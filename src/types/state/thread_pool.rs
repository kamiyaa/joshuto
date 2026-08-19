use std::collections::HashMap;
use std::thread;

/// Tracks join handles for forked/spawned child process threads, keyed by process id.
#[derive(Debug)]
pub struct ThreadPool {
    // forks of applications
    pub child_pool: HashMap<u32, thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Creates an empty thread pool.
    pub fn new() -> Self {
        Self {
            child_pool: HashMap::new(),
        }
    }
    /// Registers a spawned child process thread under `child_id`.
    pub fn push_child(&mut self, child_id: u32, handle: thread::JoinHandle<()>) {
        self.child_pool.insert(child_id, handle);
    }

    /// Removes and joins the thread for `child_id`, if tracked.
    pub fn join_child(&mut self, child_id: u32) {
        if let Some(handle) = self.child_pool.remove(&child_id) {
            let _ = handle.join();
        }
    }
}
