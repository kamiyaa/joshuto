use crate::tab::TabHomePage;
use crate::QuitAction;

#[derive(Clone, Debug)]
pub struct TabOption {
    pub _home_page: TabHomePage,
    pub _tab_quit_action: QuitAction
}

impl TabOption {
    pub fn new(_home_page: TabHomePage, _tab_quit_action: QuitAction) -> Self {
        Self { _home_page, _tab_quit_action }
    }
    pub fn home_page(&self) -> TabHomePage {
        self._home_page
    }
    pub fn tab_quit_action(&self) -> QuitAction {
        self._tab_quit_action
    }
}

impl std::default::Default for TabOption {
    fn default() -> Self {
        Self {
            _home_page: TabHomePage::Home,
            _tab_quit_action: QuitAction::Noop,
        }
    }
}
