use serde_derive::Deserialize;
use std::collections::HashMap;

use super::DEFAULT_CONFIG_FILE_PATH;
use super::{AppStyle, RawAppStyle};
use crate::config::{parse_to_config_file, TomlConfigFile};
use crate::error::JoshutoResult;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppThemeCrude {
    #[serde(default)]
    pub regular: RawAppStyle,
    #[serde(default)]
    pub selection: RawAppStyle,
    #[serde(default)]
    pub directory: RawAppStyle,
    #[serde(default)]
    pub executable: RawAppStyle,
    #[serde(default)]
    pub link: RawAppStyle,
    #[serde(default)]
    pub link_invalid: RawAppStyle,
    #[serde(default)]
    pub socket: RawAppStyle,
    #[serde(default)]
    pub ext: HashMap<String, RawAppStyle>,
}

impl From<AppThemeCrude> for AppTheme {
    fn from(crude: AppThemeCrude) -> Self {
        let selection = crude.selection.to_style_theme();
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
    pub directory: AppStyle,
    pub executable: AppStyle,
    pub link: AppStyle,
    pub link_invalid: AppStyle,
    pub socket: AppStyle,
    pub ext: HashMap<String, AppStyle>,
}

impl AppTheme {
    pub fn default_res() -> JoshutoResult<Self> {
        let crude: AppThemeCrude = toml::from_str(DEFAULT_CONFIG_FILE_PATH)?;
        Ok(Self::from(crude))
    }
}

impl TomlConfigFile for AppTheme {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<AppThemeCrude, AppTheme>(file_name).unwrap_or_default()
    }
}

impl std::default::Default for AppTheme {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        Self::default_res().unwrap()
    }
}
