use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{style_raw::AppStyleRaw, tab_raw::TabThemeRaw};

/// TOML-deserializable form of [`AppTheme`](super::AppTheme).
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
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
    pub border: AppStyleRaw,
    #[serde(default)]
    pub ext: HashMap<String, AppStyleRaw>,
    #[serde(default)]
    pub lscolors_enabled: bool,
    #[serde(default)]
    pub preview_background: String,
}
