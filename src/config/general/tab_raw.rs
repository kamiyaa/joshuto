use std::convert::From;

use serde_derive::Deserialize;

use crate::config::option::TabOption;
use crate::tab::TabHomePage;

fn default_home_page() -> String {
    "home".to_string()
}

#[derive(Clone, Debug, Deserialize)]
pub struct TabOptionRaw {
    #[serde(default = "default_home_page")]
    pub home_page: String,
}

impl std::default::Default for TabOptionRaw {
    fn default() -> Self {
        Self {
            home_page: default_home_page(),
        }
    }
}

impl From<TabOptionRaw> for TabOption {
    fn from(raw: TabOptionRaw) -> Self {
        let home_page = match raw.home_page.as_str() {
            "inherit" => TabHomePage::Inherit,
            "home" => TabHomePage::Home,
            "root" => TabHomePage::Root,
            _ => TabHomePage::Home,
        };

        Self::new(home_page)
    }
}
