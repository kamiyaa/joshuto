use std::collections::HashMap;

use super::{app_raw::AppConfigRaw, tab::TabOption};
use crate::{
    constants::config::APP_CONFIG,
    error::AppResult,
    traits::config::TomlConfigFile,
    types::{
        config_type::ConfigType,
        custom_command::CustomCommand,
        option::{display::DisplayOption, preview::PreviewOption, search::SearchOption},
    },
};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub use_trash: bool,
    pub xdg_open: bool,
    pub xdg_open_fork: bool,
    pub case_insensitive_ext: bool,
    pub watch_files: bool,
    pub custom_commands: Vec<CustomCommand>,
    pub focus_on_create: bool,
    pub mouse_support: bool,
    pub cmd_aliases: HashMap<String, String>,
    pub zoxide_update: bool,
    pub display_options: DisplayOption,
    pub preview_options: PreviewOption,
    pub search_options: SearchOption,
    pub tab_options: TabOption,
}

impl AppConfig {
    pub fn default_res() -> AppResult<Self> {
        let raw: AppConfigRaw = toml::from_str(APP_CONFIG)?;
        Ok(Self::from(raw))
    }
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        Self::default_res().unwrap()
    }
}

impl TomlConfigFile for AppConfig {
    type Raw = AppConfigRaw;

    fn get_type() -> ConfigType {
        ConfigType::App
    }
}

impl From<AppConfigRaw> for AppConfig {
    fn from(raw: AppConfigRaw) -> Self {
        Self {
            use_trash: raw.use_trash,
            xdg_open: raw.xdg_open,
            xdg_open_fork: raw.xdg_open_fork,
            case_insensitive_ext: raw.case_insensitive_ext,
            watch_files: raw.watch_files,
            cmd_aliases: raw.cmd_aliases,
            focus_on_create: raw.focus_on_create,
            mouse_support: raw.mouse_support,
            zoxide_update: raw.zoxide_update,
            display_options: DisplayOption::from(raw.display_options),
            preview_options: PreviewOption::from(raw.preview_options),
            search_options: SearchOption::from(raw.search_options),
            tab_options: TabOption::from(raw.tab_options),
            custom_commands: raw.custom_commands,
        }
    }
}
