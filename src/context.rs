use commands::FileOperationThread;
use config;
use tab::JoshutoTab;
use window::JoshutoView;

pub struct JoshutoContext {
    pub username: String,
    pub hostname: String,
    pub threads: Vec<FileOperationThread>,
    pub views: JoshutoView,
    pub curr_tab_index: usize,
    pub tabs: Vec<JoshutoTab>,

    pub config_t: config::JoshutoConfig,
}

impl JoshutoContext {
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
    pub fn curr_tab_ref(&self) -> &JoshutoTab {
        &self.tabs[self.curr_tab_index]
    }
    pub fn curr_tab_mut(&mut self) -> &mut JoshutoTab {
        &mut self.tabs[self.curr_tab_index]
    }
}
