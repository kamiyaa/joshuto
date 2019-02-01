use std::sync;
use std::thread;

use joshuto::command;
use joshuto::config;
use joshuto::tab::JoshutoTab;
use joshuto::window::JoshutoView;

pub struct JoshutoContext {
    pub username: String,
    pub hostname: String,
    pub threads: Vec<(
        sync::mpsc::Receiver<command::ProgressInfo>,
        thread::JoinHandle<i32>,
    )>,
    pub views: JoshutoView,
    pub curr_tab_index: usize,
    pub tabs: Vec<JoshutoTab>,

    pub config_t: config::JoshutoConfig,
}

impl<'a> JoshutoContext {
    pub fn new(config_t: config::JoshutoConfig) -> Self {
        let username: String = whoami::username();
        let hostname: String = whoami::hostname();

        let views: JoshutoView = JoshutoView::new(config_t.column_ratio);

        JoshutoContext {
            username,
            hostname,
            threads: Vec::new(),
            views,
            curr_tab_index: 0,
            tabs: Vec::new(),
            config_t,
        }
    }
    pub fn curr_tab_ref(&'a self) -> &'a JoshutoTab {
        &self.tabs[self.curr_tab_index]
    }
    pub fn curr_tab_mut(&'a mut self) -> &'a mut JoshutoTab {
        &mut self.tabs[self.curr_tab_index]
    }
}
