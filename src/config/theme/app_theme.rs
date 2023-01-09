use serde_derive::Deserialize;
use std::collections::HashMap;

use super::DEFAULT_CONFIG_FILE_PATH;
use super::{AppStyle, AppStyleRaw};
use crate::config::{parse_config_or_default, TomlConfigFile};
use crate::error::JoshutoResult;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppThemeRaw {
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

impl From<AppThemeRaw> for AppTheme {
    fn from(crude: AppThemeRaw) -> Self {
        let selection = crude.selection.to_style_theme();
        let visual_mode_selection = crude.visual_mode_selection.to_style_theme();
        let executable = crude.executable.to_style_theme();
        let regular = crude.regular.to_style_theme();
        let directory = crude.directory.to_style_theme();
        let link = crude.link.to_style_theme();
        let link_invalid = crude.link_invalid.to_style_theme();
        let socket = crude.socket.to_style_theme();
        let ext: HashMap<String, AppStyle> = crude
            .ext
            .iter()
            .map(|(k, v)| {
                let style = v.to_style_theme();
                (k.clone(), style)
            })
            .collect();

        Self {
            selection,
            visual_mode_selection,
            executable,
            regular,
            directory,
            link,
            link_invalid,
            socket,
            ext,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppTheme {
    pub regular: AppStyle,
    pub selection: AppStyle,
    pub visual_mode_selection: AppStyle,
    pub directory: AppStyle,
    pub executable: AppStyle,
    pub link: AppStyle,
    pub link_invalid: AppStyle,
    pub socket: AppStyle,
    pub ext: HashMap<String, AppStyle>,
}

impl AppTheme {
    pub fn default_res() -> JoshutoResult<Self> {
        let crude: AppThemeRaw = toml::from_str(DEFAULT_CONFIG_FILE_PATH)?;
        Ok(Self::from(crude))
    }
}

impl TomlConfigFile for AppTheme {
    fn get_config(file_name: &str) -> Self {
        parse_config_or_default::<AppThemeRaw, AppTheme>(file_name)
    }
}

impl std::default::Default for AppTheme {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        Self::default_res().unwrap()
    }
}
