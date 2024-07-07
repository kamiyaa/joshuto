use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub enum TabHomePage {
    #[default]
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "inherit")]
    Inherit,
    #[serde(rename = "root")]
    Root,
}
