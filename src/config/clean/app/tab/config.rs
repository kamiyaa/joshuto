use crate::{config::raw::app::display::tab::TabOptionRaw, tab::TabHomePage};

#[derive(Clone, Debug)]
pub struct TabOption {
    pub _home_page: TabHomePage,
}

impl TabOption {
    pub fn new(_home_page: TabHomePage) -> Self {
        Self { _home_page }
    }
    pub fn home_page(&self) -> TabHomePage {
        self._home_page
    }
}

impl std::default::Default for TabOption {
    fn default() -> Self {
        Self {
            _home_page: TabHomePage::Home,
        }
    }
}

impl From<TabOptionRaw> for TabOption {
    fn from(raw: TabOptionRaw) -> Self {
        let home_page = TabHomePage::from_str(raw.home_page.as_str()).unwrap_or(TabHomePage::Home);

        Self::new(home_page)
    }
}
