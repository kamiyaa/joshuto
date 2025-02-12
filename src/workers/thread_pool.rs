use std::collections::HashMap;
use std::thread;

#[derive(Debug)]
pub struct ThreadPool {
    // forks of applications
    pub child_pool: HashMap<u32, thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            child_pool: HashMap::new(),
        }
    }
    pub fn push_child(&mut self, child_id: u32, handle: thread::JoinHandle<()>) {
        self.child_pool.insert(child_id, handle);
    }

    pub fn join_child(&mut self, child_id: u32) {
        if let Some(handle) = self.child_pool.remove(&child_id) {
            let _ = handle.join();
        }
    }
}
