use serde_derive::Deserialize;

use crate::config::Flattenable;
use crate::tab::TabHomePage;

fn default_home_page() -> String {
    "home".to_string()
}

#[derive(Clone, Debug, Deserialize)]
pub struct TabRawOption {
    #[serde(default = "default_home_page")]
    home_page: String,
}

impl std::default::Default for TabRawOption {
    fn default() -> Self {
        Self {
            home_page: default_home_page(),
        }
    }
}

impl Flattenable<TabOption> for TabRawOption {
    fn flatten(self) -> TabOption {
        let home_page = match self.home_page.as_str() {
            "inherit" => TabHomePage::Inherit,
            "home" => TabHomePage::Home,
            "root" => TabHomePage::Root,
            _ => TabHomePage::Home,
        };

        TabOption::new(home_page)
    }
}

#[derive(Clone, Debug)]
pub struct TabOption {
    _home_page: TabHomePage,
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
