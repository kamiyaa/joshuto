use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

use crate::config;
use crate::io::{IOWorkerObserver, IOWorkerThread};
use crate::tab::JoshutoTab;
use crate::util::event::{Event, Events};

pub struct JoshutoContextWorker {
    queue: VecDeque<IOWorkerThread>,
    pub observer: Option<IOWorkerObserver>,
}

impl JoshutoContextWorker {
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn run_next_job(&mut self, tx: mpsc::Sender<Event>) {
        let worker = self.queue.pop_front().unwrap();
        let src = worker.paths[0].clone();
        let dest = worker.dest.clone();
        let file_op = worker.options.kind;
        let handle = thread::spawn(move || {
            let (wtx, wrx) = mpsc::channel();
            let worker_handle = thread::spawn(move || worker.start(wtx));

            while let Ok(progress) = wrx.recv() {
                tx.send(Event::IOWorkerProgress((file_op, progress)));
            }
            let result = worker_handle.join().unwrap();
            let _ = tx.send(Event::IOWorkerResult((file_op, result)));
        });
        let observer = IOWorkerObserver::new(handle, src, dest);
        self.observer = Some(observer);
    }

    pub fn is_busy(&self) -> bool {
        self.observer.is_some()
    }
}

impl std::default::Default for JoshutoContextWorker {
    fn default() -> Self {
        Self {
            queue: VecDeque::with_capacity(10),
            observer: None,
        }
    }
}

pub struct JoshutoContext {
    pub exit: bool,
    pub curr_tab_index: usize,
    pub tabs: Vec<JoshutoTab>,
    pub message_queue: VecDeque<String>,
    pub events: Events,
    pub worker: JoshutoContextWorker,
    pub config_t: config::JoshutoConfig,
}

impl JoshutoContext {
    pub fn new(config_t: config::JoshutoConfig) -> Self {
        Self {
            exit: false,
            curr_tab_index: 0,
            tabs: Vec::new(),
            message_queue: VecDeque::with_capacity(4),
            events: Events::new(),
            worker: JoshutoContextWorker::default(),
            config_t,
        }
    }
    pub fn curr_tab_ref(&self) -> &JoshutoTab {
        &self.tabs[self.curr_tab_index]
    }
    pub fn curr_tab_mut(&mut self) -> &mut JoshutoTab {
        &mut self.tabs[self.curr_tab_index]
    }
    pub fn push_tab(&mut self, tab: JoshutoTab) {
        self.tabs.push(tab);
        self.curr_tab_index = self.tabs.len() - 1;
    }

    pub fn push_msg(&mut self, msg: String) {
        self.message_queue.push_back(msg);
    }

    pub fn push_worker_thread(&mut self, thread: IOWorkerThread) {
        self.worker.queue.push_back(thread);
    }
}
