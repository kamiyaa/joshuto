use serde::Deserialize;
use std::collections::HashMap;

use super::style::AppStyleRaw;
use super::tab::TabThemeRaw;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppThemeRaw {
    #[serde(default)]
    pub tabs: TabThemeRaw,
    #[serde(default)]
    pub regular: AppStyleRaw,
    #[serde(default)]
    pub selection: AppStyleRaw,
    #[serde(default)]
    pub visual_mode_selection: AppStyleRaw,
    #[serde(default)]
    pub directory: AppStyleRaw,
    #[serde(default)]
    pub executable: AppStyleRaw,
    #[serde(default)]
    pub link: AppStyleRaw,
    #[serde(default)]
    pub link_invalid: AppStyleRaw,
    #[serde(default)]
    pub socket: AppStyleRaw,
    #[serde(default)]
    pub ext: HashMap<String, AppStyleRaw>,
}
