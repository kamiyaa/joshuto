use std::convert::From;

use serde_derive::Deserialize;

use crate::tab::TabHomePage;

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
