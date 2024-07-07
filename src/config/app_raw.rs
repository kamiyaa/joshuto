use std::collections::HashMap;

use serde::Deserialize;

use super::display_raw::DisplayOptionRaw;
use super::preview::preview_option_raw::PreviewOptionRaw;
use super::tab::TabOption;

use crate::types::custom_command::CustomCommand;
use crate::types::option::search::SearchOption;
use crate::utils::serde::default_true;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigRaw {
    #[serde(default = "default_true")]
    pub use_trash: bool,
    #[serde(default)]
    pub xdg_open: bool,
    #[serde(default)]
    pub case_insensitive_ext: bool,
    #[serde(default)]
    pub xdg_open_fork: bool,
    #[serde(default = "default_true")]
    pub watch_files: bool,
    #[serde(default = "default_true")]
    pub focus_on_create: bool,
    #[serde(default = "default_true")]
    pub mouse_support: bool,
    #[serde(default)]
    pub zoxide_update: bool,
    #[serde(default)]
    pub cmd_aliases: HashMap<String, String>,
    #[serde(default, rename = "display")]
    pub display_options: DisplayOptionRaw,
    #[serde(default, rename = "preview")]
    pub preview_options: PreviewOptionRaw,
    #[serde(default, rename = "search")]
    pub search_options: SearchOption,
    #[serde(default, rename = "tab")]
    pub tab_options: TabOption,
    #[serde(default)]
    pub custom_commands: Vec<CustomCommand>,
}
