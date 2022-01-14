use std::convert::From;

use serde_derive::Deserialize;

use crate::config::option::{DisplayOption, PreviewOption, TabOption};
use crate::config::{parse_to_config_file, AppConfig, TomlConfigFile};

use super::display_crude::DisplayOptionCrude;
use super::preview_crude::PreviewOptionCrude;
use super::tab_crude::TabOptionCrude;

const fn default_true() -> bool {
    true
}
const fn default_scroll_offset() -> usize {
    6
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigCrude {
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
    #[serde(default, rename = "display")]
    pub display_options: DisplayOptionCrude,
    #[serde(default, rename = "preview")]
    pub preview_options: PreviewOptionCrude,
    #[serde(default, rename = "tab")]
    pub tab_options: TabOptionCrude,
}

impl From<AppConfigCrude> for AppConfig {
    fn from(crude: AppConfigCrude) -> Self {
        Self {
            use_trash: crude.use_trash,
            xdg_open: crude.xdg_open,
            xdg_open_fork: crude.xdg_open_fork,
            watch_files: crude.watch_files,
            _display_options: DisplayOption::from(crude.display_options),
            _preview_options: PreviewOption::from(crude.preview_options),
            _tab_options: TabOption::from(crude.tab_options),
        }
    }
}

impl TomlConfigFile for AppConfig {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<AppConfigCrude, AppConfig>(file_name).unwrap_or_else(Self::default)
    }
}
