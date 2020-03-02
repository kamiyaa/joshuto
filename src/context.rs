use std::collections::VecDeque;
use std::sync::mpsc;

use crate::config;
use crate::io::IOWorkerThread;
use crate::tab::JoshutoTab;
use crate::util::event::Events;

pub struct JoshutoContext {
    pub exit: bool,
    pub curr_tab_index: usize,
    pub tabs: Vec<JoshutoTab>,
    pub worker_queue: VecDeque<IOWorkerThread>,
    pub trx: (mpsc::SyncSender<u64>, mpsc::Receiver<u64>),

    pub worker_msg: Option<String>,
    pub message_queue: VecDeque<String>,
    pub events: Events,

    pub config_t: config::JoshutoConfig,
}

impl JoshutoContext {
    pub fn new(config_t: config::JoshutoConfig) -> Self {
        JoshutoContext {
            exit: false,
            curr_tab_index: 0,
            tabs: Vec::new(),
            worker_queue: VecDeque::with_capacity(10),
            trx: mpsc::sync_channel::<u64>(1),
            worker_msg: None,
            message_queue: VecDeque::with_capacity(4),
            events: Events::new(),

            config_t,
        }
    }
    pub fn curr_tab_ref(&self) -> &JoshutoTab {
        &self.tabs[self.curr_tab_index]
    }
    pub fn curr_tab_mut(&mut self) -> &mut JoshutoTab {
        &mut self.tabs[self.curr_tab_index]
    }
    pub fn add_new_worker(&mut self, thread: IOWorkerThread) {
        self.worker_queue.push_back(thread);
    }

    pub fn push_tab(&mut self, tab: JoshutoTab) {
        self.tabs.push(tab);
        self.curr_tab_index = self.tabs.len() - 1;
    }
}
