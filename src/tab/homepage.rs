use serde::{Deserialize, Serialize};

/// Where a newly-opened tab starts, when not overridden by [`NewTabMode`](super::NewTabMode).
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
