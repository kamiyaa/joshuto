use serde::{Deserialize, Serialize};

use crate::tab::TabHomePage;

use super::theme::tab_raw::TabStyle;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TabOption {
    #[serde(default)]
    pub style: TabStyle,
    #[serde(default)]
    pub home_page: TabHomePage,
}
