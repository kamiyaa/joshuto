use serde_derive::Deserialize;

use super::preview::{PreviewOption, PreviewRawOption};
use super::tab::{TabOption, TabRawOption};
use super::DisplayRawOption;

use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::util::display::DisplayOption;
use crate::util::sort;

const fn default_true() -> bool {
    true
}
const fn default_scroll_offset() -> usize {
    6
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawAppConfig {
    #[serde(default = "default_scroll_offset")]
    scroll_offset: usize,
    #[serde(default = "default_true")]
    use_trash: bool,
    #[serde(default)]
    xdg_open: bool,
    #[serde(default, rename = "display")]
    display_options: DisplayRawOption,
    #[serde(default, rename = "preview")]
    preview_options: PreviewRawOption,
    #[serde(default, rename = "tab")]
    tab_options: TabRawOption,
}

impl Flattenable<AppConfig> for RawAppConfig {
    fn flatten(self) -> AppConfig {
        AppConfig {
            scroll_offset: self.scroll_offset,
            use_trash: self.use_trash,
            xdg_open: self.xdg_open,
            _display_options: self.display_options.flatten(),
            _preview_options: self.preview_options.flatten(),
            _tab_options: self.tab_options.flatten(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub scroll_offset: usize,
    pub use_trash: bool,
    pub xdg_open: bool,
    _display_options: DisplayOption,
    _preview_options: PreviewOption,
    _tab_options: TabOption,
}

impl AppConfig {
    pub fn display_options_ref(&self) -> &DisplayOption {
        &self._display_options
    }
    pub fn display_options_mut(&mut self) -> &mut DisplayOption {
        &mut self._display_options
    }

    pub fn preview_options_ref(&self) -> &PreviewOption {
        &self._preview_options
    }
    pub fn preview_options_mut(&mut self) -> &mut PreviewOption {
        &mut self._preview_options
    }

    pub fn sort_options_ref(&self) -> &sort::SortOption {
        self.display_options_ref().sort_options_ref()
    }
    pub fn sort_options_mut(&mut self) -> &mut sort::SortOption {
        self.display_options_mut().sort_options_mut()
    }

    pub fn tab_options_ref(&self) -> &TabOption {
        &self._tab_options
    }
    pub fn tab_options_mut(&mut self) -> &mut TabOption {
        &mut self._tab_options
    }
}

impl ConfigStructure for AppConfig {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<RawAppConfig, AppConfig>(file_name).unwrap_or_else(Self::default)
    }
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            scroll_offset: default_scroll_offset(),
            use_trash: true,
            xdg_open: false,
            _display_options: DisplayOption::default(),
            _preview_options: PreviewOption::default(),
            _tab_options: TabOption::default(),
        }
    }
}
