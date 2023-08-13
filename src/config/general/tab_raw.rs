use std::convert::From;

use serde_derive::Deserialize;

use crate::config::option::{TabBarDisplayMode, TabOption};
use crate::tab::TabHomePage;

fn default_home_page() -> String {
    "home".to_string()
}

const fn default_max_len() -> usize {
    16
}

#[derive(Clone, Debug, Deserialize)]
pub struct TabOptionRaw {
    #[serde(default = "default_home_page")]
    pub home_page: String,
    #[serde(default)]
    pub display_mode: TabBarDisplayMode,
    #[serde(default = "default_max_len")]
    pub max_len: usize,
}

impl std::default::Default for TabOptionRaw {
    fn default() -> Self {
        Self {
            home_page: default_home_page(),
            display_mode: TabBarDisplayMode::default(),
            max_len: 16,
        }
    }
}

impl From<TabOptionRaw> for TabOption {
    fn from(raw: TabOptionRaw) -> Self {
        let home_page = TabHomePage::from_str(raw.home_page.as_str()).unwrap_or(TabHomePage::Home);

        Self::new(home_page, raw.display_mode, raw.max_len)
    }
}
