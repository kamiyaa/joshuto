use serde::{Deserialize, Serialize};

use crate::tab::TabHomePage;

fn default_home_page() -> TabHomePage {
    TabHomePage::default()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TabOption {
    #[serde(default = "default_home_page")]
    pub home_page: TabHomePage,
}

impl std::default::Default for TabOption {
    fn default() -> Self {
        Self {
            home_page: default_home_page(),
        }
    }
}
