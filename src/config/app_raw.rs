use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

use crate::config::{
    AppTabOption, CustomCommand, DisplayOptionRaw, PreviewOptionRaw, SearchOption,
};

use super::utils::{default_shell, default_true};

/// TOML-deserializable form of [`AppConfig`](super::app::AppConfig).
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
    pub tab_options: AppTabOption,
    #[serde(default)]
    pub custom_commands: Vec<CustomCommand>,
    #[serde(default = "default_shell")]
    pub shell: PathBuf,
}
