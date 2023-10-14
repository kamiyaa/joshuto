use std::collections::HashMap;

use crate::{
    config::{
        parse_config_or_default,
        raw::app::{AppConfigRaw, CustomCommand},
        TomlConfigFile,
    },
    error::AppResult,
};

use super::{
    display::DisplayOption, preview::PreviewOption, search::SearchOption, tab::TabOption,
    DEFAULT_CONFIG_FILE_PATH,
};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub use_trash: bool,
    pub xdg_open: bool,
    pub xdg_open_fork: bool,
    pub watch_files: bool,
    pub custom_commands: Vec<CustomCommand>,
    pub cmd_aliases: HashMap<String, String>,
    pub _display_options: DisplayOption,
    pub _preview_options: PreviewOption,
    pub _search_options: SearchOption,
    pub _tab_options: TabOption,
}

impl AppConfig {
    pub fn default_res() -> AppResult<Self> {
        let raw: AppConfigRaw = toml::from_str(DEFAULT_CONFIG_FILE_PATH)?;
        Ok(Self::from(raw))
    }

    pub fn display_options_ref(&self) -> &DisplayOption {
        &self._display_options
    }
    pub fn display_options_mut(&mut self) -> &mut DisplayOption {
        &mut self._display_options
    }

    pub fn preview_options_ref(&self) -> &PreviewOption {
        &self._preview_options
    }
    pub fn _preview_options_mut(&mut self) -> &mut PreviewOption {
        &mut self._preview_options
    }

    pub fn search_options_ref(&self) -> &SearchOption {
        &self._search_options
    }

    pub fn search_options_mut(&mut self) -> &mut SearchOption {
        &mut self._search_options
    }

    pub fn tab_options_ref(&self) -> &TabOption {
        &self._tab_options
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
    fn get_config(file_name: &str) -> Self {
        parse_config_or_default::<AppConfigRaw, AppConfig>(file_name)
    }
}

impl From<AppConfigRaw> for AppConfig {
    fn from(raw: AppConfigRaw) -> Self {
        Self {
            use_trash: raw.use_trash,
            xdg_open: raw.xdg_open,
            xdg_open_fork: raw.xdg_open_fork,
            watch_files: raw.watch_files,
            cmd_aliases: raw.cmd_aliases,
            _display_options: DisplayOption::from(raw.display_options),
            _preview_options: PreviewOption::from(raw.preview_options),
            _search_options: SearchOption::from(raw.search_options),
            _tab_options: TabOption::from(raw.tab_options),
            custom_commands: raw.custom_commands,
        }
    }
}
