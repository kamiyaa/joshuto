use crate::commands::FileOperationThread;
use crate::config;
use crate::tab::JoshutoTab;

pub struct JoshutoContext {
    pub threads: Vec<FileOperationThread<u64, fs_extra::TransitProcess>>,
    pub curr_tab_index: usize,
    pub tabs: Vec<JoshutoTab>,
    pub exit: bool,

    pub config_t: config::JoshutoConfig,
}

impl JoshutoContext {
    pub fn new(config_t: config::JoshutoConfig) -> Self {
        JoshutoContext {
            threads: Vec::new(),
            curr_tab_index: 0,
            tabs: Vec::new(),
            exit: false,
            config_t,
        }
    }
    pub fn curr_tab_ref(&self) -> &JoshutoTab {
        &self.tabs[self.curr_tab_index]
    }
    pub fn curr_tab_mut(&mut self) -> &mut JoshutoTab {
        &mut self.tabs[self.curr_tab_index]
    }
}
