use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

use crate::config;
use crate::context::{LocalStateContext, TabContext};
use crate::io::{IOWorkerObserver, IOWorkerProgress, IOWorkerThread};
use crate::util::event::{Events, JoshutoEvent};

pub struct JoshutoContext {
    pub exit: bool,
    config: config::JoshutoConfig,
    events: Events,
    tab_context: TabContext,
    local_state: Option<LocalStateContext>,
    search_state: Option<String>,
    message_queue: VecDeque<String>,
    worker_queue: VecDeque<IOWorkerThread>,
    worker: Option<IOWorkerObserver>,
}

impl JoshutoContext {
    pub fn new(config: config::JoshutoConfig) -> Self {
        Self {
            exit: false,
            events: Events::new(),
            tab_context: TabContext::new(),
            local_state: None,
            search_state: None,
            message_queue: VecDeque::with_capacity(4),
            worker_queue: VecDeque::new(),
            worker: None,
            config,
        }
    }

    pub fn config_ref(&self) -> &config::JoshutoConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut config::JoshutoConfig {
        &mut self.config
    }

    pub fn tab_context_ref(&self) -> &TabContext {
        &self.tab_context
    }
    pub fn tab_context_mut(&mut self) -> &mut TabContext {
        &mut self.tab_context
    }

    pub fn message_queue_ref(&self) -> &VecDeque<String> {
        &self.message_queue
    }
    pub fn push_msg(&mut self, msg: String) {
        self.message_queue.push_back(msg);
    }
    pub fn pop_msg(&mut self) -> Option<String> {
        self.message_queue.pop_front()
    }

    // event related
    pub fn poll_event(&self) -> Result<JoshutoEvent, mpsc::RecvError> {
        self.events.next()
    }
    pub fn get_event_tx(&self) -> mpsc::Sender<JoshutoEvent> {
        self.events.event_tx.clone()
    }
    pub fn flush_event(&self) {
        self.events.flush();
    }

    // local state related
    pub fn set_local_state(&mut self, state: LocalStateContext) {
        self.local_state = Some(state);
    }
    pub fn take_local_state(&mut self) -> Option<LocalStateContext> {
        self.local_state.take()
    }

    pub fn set_search_state(&mut self, pattern: String) {
        self.search_state = Some(pattern);
    }

    pub fn get_search_state(&self) -> Option<&String> {
        self.search_state.as_ref()
    }

    // worker related
    pub fn add_worker(&mut self, thread: IOWorkerThread) {
        self.worker_queue.push_back(thread);
    }
    pub fn worker_is_busy(&self) -> bool {
        self.worker.is_some()
    }
    pub fn worker_is_empty(&self) -> bool {
        self.worker_queue.is_empty()
    }

    pub fn worker_iter(&self) -> Iter<IOWorkerThread> {
        self.worker_queue.iter()
    }

    pub fn worker_ref(&self) -> Option<&IOWorkerObserver> {
        self.worker.as_ref()
    }

    pub fn set_worker_progress(&mut self, res: IOWorkerProgress) {
        if let Some(s) = self.worker.as_mut() {
            s.set_progress(res);
        }
    }

    pub fn update_worker_msg(&mut self) {
        if let Some(s) = self.worker.as_mut() {
            s.update_msg();
        }
    }
    pub fn worker_msg(&self) -> Option<&str> {
        let worker = self.worker.as_ref()?;
        Some(worker.get_msg())
    }

    pub fn start_next_job(&mut self) {
        let tx = self.get_event_tx();

        if let Some(worker) = self.worker_queue.pop_front() {
            let src = worker.paths[0].parent().unwrap().to_path_buf();
            let dest = worker.dest.clone();
            let handle = thread::spawn(move || {
                let (wtx, wrx) = mpsc::channel();
                // start worker
                let worker_handle = thread::spawn(move || worker.start(wtx));
                // relay worker info to event loop
                while let Ok(progress) = wrx.recv() {
                    let _ = tx.send(JoshutoEvent::IOWorkerProgress(progress));
                }
                let result = worker_handle.join();

                match result {
                    Ok(res) => {
                        let _ = tx.send(JoshutoEvent::IOWorkerResult(res));
                    }
                    Err(_) => {
                        let err = std::io::Error::new(std::io::ErrorKind::Other, "Sending Error");
                        let _ = tx.send(JoshutoEvent::IOWorkerResult(Err(err)));
                    }
                }
            });
            let observer = IOWorkerObserver::new(handle, src, dest);
            self.worker = Some(observer);
        }
    }

    pub fn remove_job(&mut self) -> Option<IOWorkerObserver> {
        self.worker.take()
    }
}
