use std::collections::VecDeque;
use std::sync::mpsc;

use crate::config;
use crate::context::{LocalStateContext, TabContext, WorkerContext};
use crate::util::event::{AppEvent, Events};
use crate::util::search::SearchPattern;

pub struct AppContext {
    pub exit: bool,
    // app config
    config: config::AppConfig,
    // event loop querying
    events: Events,
    // context related to tabs
    tab_context: TabContext,
    // context related to local file state
    local_state: Option<LocalStateContext>,
    // context related to searching
    search_context: Option<SearchPattern>,
    // message queue for displaying messages
    message_queue: VecDeque<String>,
    // context related to io workers
    worker_context: WorkerContext,
}

impl AppContext {
    pub fn new(config: config::AppConfig) -> Self {
        let events = Events::new();
        let event_tx = events.event_tx.clone();
        Self {
            exit: false,
            events,
            tab_context: TabContext::new(),
            local_state: None,
            search_context: None,
            message_queue: VecDeque::with_capacity(4),
            worker_context: WorkerContext::new(event_tx),
            config,
        }
    }

    // event related
    pub fn poll_event(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.events.next()
    }
    pub fn flush_event(&self) {
        self.events.flush();
    }

    pub fn config_ref(&self) -> &config::AppConfig {
        &self.config
    }
    pub fn config_mut(&mut self) -> &mut config::AppConfig {
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

    // local state related
    pub fn set_local_state(&mut self, state: LocalStateContext) {
        self.local_state = Some(state);
    }
    pub fn take_local_state(&mut self) -> Option<LocalStateContext> {
        self.local_state.take()
    }

    pub fn get_search_context(&self) -> Option<&SearchPattern> {
        self.search_context.as_ref()
    }
    pub fn set_search_context(&mut self, pattern: SearchPattern) {
        self.search_context = Some(pattern);
    }

    pub fn worker_context_ref(&self) -> &WorkerContext {
        &self.worker_context
    }
    pub fn worker_context_mut(&mut self) -> &mut WorkerContext {
        &mut self.worker_context
    }
}
