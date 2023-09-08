use serde::Deserialize;

use super::style::AppStyleRaw;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct TabThemeRaw {
    #[serde(default)]
    pub inactive: AppStyleRaw,
    #[serde(default)]
    pub active: AppStyleRaw,
}
