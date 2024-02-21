use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    lock: Mutex<usize>,
    cvar: Condvar,
}

pub struct SemaphoreGuard<'a> {
    sem: &'a Semaphore,
}

impl Semaphore {
    /// Creates a new semaphore with the initial count specified
    pub fn new(count: usize) -> Semaphore {
        Semaphore {
            lock: Mutex::new(count),
            cvar: Condvar::new(),
        }
    }

    /// Acquires a resource of this semaphore, blocking the current thread
    pub fn acquire(&self) {
        let mut count = self.lock.lock().unwrap();
        while *count == 0 {
            count = self.cvar.wait(count).unwrap();
        }
        *count -= 1;
    }

    /// Release a resource from this semaphore
    pub fn release(&self) {
        *self.lock.lock().unwrap() += 1;
        self.cvar.notify_one();
    }

    /// Acquires a resource of this semaphore, returning an RAII guard
    pub fn access(&self) -> SemaphoreGuard {
        self.acquire();
        SemaphoreGuard { sem: self }
    }
}

impl<'a> Drop for SemaphoreGuard<'a> {
    fn drop(&mut self) {
        self.sem.release();
    }
}
