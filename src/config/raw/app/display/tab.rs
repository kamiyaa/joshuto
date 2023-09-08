use serde::Deserialize;

use crate::config::clean::app::tab::TabBarDisplayMode;

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
