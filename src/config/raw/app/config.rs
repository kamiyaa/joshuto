use std::collections::HashMap;

use serde::Deserialize;

use super::display::preview::PreviewOptionRaw;
use super::display::search::SearchOptionRaw;
use super::display::tab::TabOptionRaw;
use super::display::DisplayOptionRaw;

const fn default_true() -> bool {
    true
}
const fn default_scroll_offset() -> usize {
    6
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomCommand {
    pub name: String,
    pub command: String,
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
    pub search_options: SearchOptionRaw,
    #[serde(default, rename = "tab")]
    pub tab_options: TabOptionRaw,
    #[serde(default)]
    pub custom_commands: Vec<CustomCommand>,
}
