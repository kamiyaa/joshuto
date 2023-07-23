use std::collections::HashMap;
use std::convert::From;

use serde_derive::Deserialize;

use crate::config::option::{DisplayOption, PreviewOption, TabOption};
use crate::config::{parse_config_or_default, AppConfig, TomlConfigFile};

use super::display_raw::DisplayOptionRaw;
use super::preview_raw::PreviewOptionRaw;
use super::tab_raw::TabOptionRaw;

const fn default_true() -> bool {
    true
}
const fn default_scroll_offset() -> usize {
    6
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigRaw {
    #[serde(default = "default_scroll_offset")]
    pub scroll_offset: usize,
    #[serde(default = "default_true")]
    pub use_trash: bool,
    #[serde(default)]
    pub xdg_open: bool,
    #[serde(default)]
    pub xdg_open_fork: bool,
    #[serde(default = "default_true")]
    pub watch_files: bool,
    #[serde(default)]
    pub cmd_aliases: HashMap<String, String>,
    #[serde(default, rename = "display")]
    pub display_options: DisplayOptionRaw,
    #[serde(default, rename = "preview")]
    pub preview_options: PreviewOptionRaw,
    #[serde(default, rename = "tab")]
    pub tab_options: TabOptionRaw,
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
            _tab_options: TabOption::from(raw.tab_options),
        }
    }
}

impl TomlConfigFile for AppConfig {
    fn get_config(file_name: &str) -> Self {
        parse_config_or_default::<AppConfigRaw, AppConfig>(file_name)
    }
}
