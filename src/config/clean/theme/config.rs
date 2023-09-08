use std::collections::HashMap;

use crate::config::raw::theme::AppThemeRaw;
use crate::config::{parse_config_or_default, TomlConfigFile};
use crate::error::AppResult;

use super::style::AppStyle;
use super::tab::TabTheme;
use super::DEFAULT_CONFIG_FILE_PATH;

#[derive(Clone, Debug)]
pub struct AppTheme {
    pub tabs: TabTheme,
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
    pub fn default_res() -> AppResult<Self> {
        let raw: AppThemeRaw = toml::from_str(DEFAULT_CONFIG_FILE_PATH)?;
        Ok(Self::from(raw))
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

impl From<AppThemeRaw> for AppTheme {
    fn from(raw: AppThemeRaw) -> Self {
        let tabs = raw.tabs;
        let selection = raw.selection.to_style_theme();
        let visual_mode_selection = raw.visual_mode_selection.to_style_theme();
        let executable = raw.executable.to_style_theme();
        let regular = raw.regular.to_style_theme();
        let directory = raw.directory.to_style_theme();
        let link = raw.link.to_style_theme();
        let link_invalid = raw.link_invalid.to_style_theme();
        let socket = raw.socket.to_style_theme();
        let ext: HashMap<String, AppStyle> = raw
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
            tabs: TabTheme::from(tabs),
        }
    }
}
