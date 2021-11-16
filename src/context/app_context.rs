use std::sync::mpsc;

use crate::config;
use crate::context::{
    CommandLineContext, LocalStateContext, MessageQueue, PreviewContext, TabContext, WorkerContext,
};
use crate::event::{AppEvent, Events};
use crate::util::search::SearchPattern;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuitType {
    DoNot,
    Normal,
    Force,
    ToCurrentDirectory,
}

pub struct AppContext {
    pub quit: QuitType,
    // event loop querying
    pub events: Events,
    // choose file instead of opening it
    pub choosefiles: bool,
    // app config
    config: config::AppConfig,
    // context related to tabs
    tab_context: TabContext,
    // context related to local file state
    local_state: Option<LocalStateContext>,
    // context related to searching
    search_context: Option<SearchPattern>,
    // message queue for displaying messages
    message_queue: MessageQueue,
    // context related to io workers
    worker_context: WorkerContext,
    // context related to previews
    preview_context: PreviewContext,
    // context related to command line
    commandline_context: CommandLineContext,
}

impl AppContext {
    pub fn new(config: config::AppConfig, choosefiles: bool) -> Self {
        let events = Events::new();
        let event_tx = events.event_tx.clone();

        let mut commandline_context = CommandLineContext::new();
        commandline_context.history_mut().set_max_len(20);
        Self {
            quit: QuitType::DoNot,
            events,
            choosefiles,
            tab_context: TabContext::new(),
            local_state: None,
            search_context: None,
            message_queue: MessageQueue::new(),
            worker_context: WorkerContext::new(event_tx),
            preview_context: PreviewContext::new(),
            commandline_context,
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
    pub fn clone_event_tx(&self) -> mpsc::Sender<AppEvent> {
        self.events.event_tx.clone()
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

    pub fn message_queue_ref(&self) -> &MessageQueue {
        &self.message_queue
    }
    pub fn message_queue_mut(&mut self) -> &mut MessageQueue {
        &mut self.message_queue
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

    pub fn preview_context_ref(&self) -> &PreviewContext {
        &self.preview_context
    }
    pub fn preview_context_mut(&mut self) -> &mut PreviewContext {
        &mut self.preview_context
    }

    pub fn worker_context_ref(&self) -> &WorkerContext {
        &self.worker_context
    }
    pub fn worker_context_mut(&mut self) -> &mut WorkerContext {
        &mut self.worker_context
    }

    pub fn commandline_context_ref(&self) -> &CommandLineContext {
        &self.commandline_context
    }

    pub fn commandline_context_mut(&mut self) -> &mut CommandLineContext {
        &mut self.commandline_context
    }
}
