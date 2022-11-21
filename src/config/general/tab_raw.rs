use std::convert::From;

use serde_derive::Deserialize;

use crate::config::option::TabOption;
use crate::tab::TabHomePage;
use crate::QuitAction;

fn default_home_page() -> String {
    "home".to_string()
}

fn default_tab_quit_action() -> String {
    "noop".to_string()
}

#[derive(Clone, Debug, Deserialize)]
pub struct TabOptionRaw {
    #[serde(default = "default_home_page")]
    pub home_page: String,
    #[serde(default = "default_tab_quit_action")]
    pub tab_quit_action: String,
}

impl std::default::Default for TabOptionRaw {
    fn default() -> Self {
        Self {
            home_page: default_home_page(),
            tab_quit_action: default_tab_quit_action(),
        }
    }
}

impl From<TabOptionRaw> for TabOption {
    fn from(raw: TabOptionRaw) -> Self {
        let home_page =
            TabHomePage::from_str(raw.home_page.as_str()).unwrap_or_else(|| TabHomePage::Home);
        let tab_quit_action =
            QuitAction::from_str(&raw.tab_quit_action.as_str()).unwrap_or_else(|| QuitAction::Noop);

        Self::new(home_page, tab_quit_action)
    }
}
