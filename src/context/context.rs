use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

use crate::config;
use crate::context::{LocalStateContext, TabContext};
use crate::io::{IOWorkerObserver, IOWorkerThread};
use crate::util::event::{Event, Events};

pub struct JoshutoContext {
    pub exit: bool,
    pub config_t: config::JoshutoConfig,
    events: Events,
    tab_context: TabContext,
    local_state: Option<LocalStateContext>,
    message_queue: VecDeque<String>,
    worker_queue: VecDeque<IOWorkerThread>,
    worker: Option<IOWorkerObserver>,
}

impl JoshutoContext {
    pub fn new(config_t: config::JoshutoConfig) -> Self {
        Self {
            exit: false,
            events: Events::new(),
            tab_context: TabContext::new(),
            local_state: None,
            message_queue: VecDeque::with_capacity(4),
            worker_queue: VecDeque::new(),
            worker: None,
            config_t,
        }
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
    pub fn poll_event(&self) -> Result<Event, mpsc::RecvError> {
        self.events.next()
    }
    pub fn get_event_tx(&self) -> mpsc::Sender<Event> {
        self.events.event_tx.clone()
    }
    pub fn flush_event(&self) {
        self.events.flush();
    }

    // local state related
    pub fn set_local_state(&mut self, state: LocalStateContext) {
        self.local_state = Some(state);
    }
    pub fn get_local_state(&self) -> Option<&LocalStateContext> {
        self.local_state.as_ref()
    }
    pub fn take_local_state(&mut self) -> Option<LocalStateContext> {
        self.local_state.take()
    }

    // worker related
    pub fn add_worker(&mut self, thread: IOWorkerThread) {
        self.worker_queue.push_back(thread);
    }
    pub fn worker_is_busy(&self) -> bool {
        self.worker.is_some()
    }
    pub fn worker_len(&self) -> usize {
        self.worker_queue.len()
    }
    pub fn worker_is_empty(&self) -> bool {
        self.worker_queue.is_empty()
    }
    pub fn set_worker_msg(&mut self, msg: String) {
        if let Some(s) = self.worker.as_mut() {
            s.set_msg(msg);
        }
    }
    pub fn worker_msg(&self) -> Option<&String> {
        self.worker.as_ref().and_then(|s| s.get_msg())
    }

    pub fn start_next_job(&mut self) {
        let tx = self.get_event_tx();

        if let Some(worker) = self.worker_queue.pop_front() {
            let src = worker.paths[0].clone();
            let dest = worker.dest.clone();
            let file_op = worker.options.kind;
            let handle = thread::spawn(move || {
                let (wtx, wrx) = mpsc::channel();
                // start worker
                let worker_handle = thread::spawn(move || worker.start(wtx));
                // relay worker info to event loop
                while let Ok(progress) = wrx.recv() {
                    tx.send(Event::IOWorkerProgress((file_op, progress)));
                }
                let result = worker_handle.join();
                match result {
                    Ok(res) => {
                        let _ = tx.send(Event::IOWorkerResult((file_op, res)));
                    }
                    Err(e) => {
                        let err = std::io::Error::new(std::io::ErrorKind::Other, "Sending Error");
                        let _ = tx.send(Event::IOWorkerResult((file_op, Err(err))));
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
