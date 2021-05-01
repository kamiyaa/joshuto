use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

use crate::config;
use crate::context::{LocalStateContext, TabContext};
use crate::io::{IoWorkerObserver, IoWorkerProgress, IoWorkerThread};
use crate::util::display::DisplayOption;
use crate::util::event::{AppEvent, Events};
use crate::util::search::SearchPattern;
use crate::util::sort;

pub struct AppContext {
    pub exit: bool,
    config: config::AppConfig,
    events: Events,
    tab_context: TabContext,
    local_state: Option<LocalStateContext>,
    search_state: Option<SearchPattern>,
    message_queue: VecDeque<String>,
    worker_queue: VecDeque<IoWorkerThread>,
    worker: Option<IoWorkerObserver>,
}

impl AppContext {
    pub fn new(config: config::AppConfig) -> Self {
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

    pub fn config_ref(&self) -> &config::AppConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut config::AppConfig {
        &mut self.config
    }

    pub fn display_options_ref(&self) -> &DisplayOption {
        self.config_ref().display_options_ref()
    }

    pub fn display_options_mut(&mut self) -> &mut DisplayOption {
        self.config_mut().display_options_mut()
    }

    pub fn sort_options_ref(&self) -> &sort::SortOption {
        self.config_ref().display_options_ref().sort_options_ref()
    }

    pub fn sort_options_mut(&mut self) -> &mut sort::SortOption {
        self.config_mut().display_options_mut().sort_options_mut()
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
    pub fn poll_event(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.events.next()
    }
    pub fn get_event_tx(&self) -> mpsc::Sender<AppEvent> {
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

    pub fn set_search_state(&mut self, pattern: SearchPattern) {
        self.search_state = Some(pattern);
    }

    pub fn get_search_state(&self) -> Option<&SearchPattern> {
        self.search_state.as_ref()
    }

    // worker related
    pub fn add_worker(&mut self, thread: IoWorkerThread) {
        self.worker_queue.push_back(thread);
    }
    pub fn worker_is_busy(&self) -> bool {
        self.worker.is_some()
    }
    pub fn worker_is_empty(&self) -> bool {
        self.worker_queue.is_empty()
    }

    pub fn worker_iter(&self) -> Iter<IoWorkerThread> {
        self.worker_queue.iter()
    }

    pub fn worker_ref(&self) -> Option<&IoWorkerObserver> {
        self.worker.as_ref()
    }

    pub fn set_worker_progress(&mut self, res: IoWorkerProgress) {
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

    pub fn remove_job(&mut self) -> Option<IoWorkerObserver> {
        self.worker.take()
    }
}
