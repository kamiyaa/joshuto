use super::DEFAULT_CONFIG_FILE_PATH;

use super::app_crude::AppConfigCrude;
use crate::config::option::{DisplayOption, PreviewOption, SortOption, TabOption};
use crate::error::JoshutoResult;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub use_trash: bool,
    pub xdg_open: bool,
    pub _display_options: DisplayOption,
    pub _preview_options: PreviewOption,
    pub _tab_options: TabOption,
}

impl AppConfig {
    pub fn default_res() -> JoshutoResult<Self> {
        let crude: AppConfigCrude = toml::from_str(DEFAULT_CONFIG_FILE_PATH)?;
        Ok(Self::from(crude))
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
    pub fn preview_options_mut(&mut self) -> &mut PreviewOption {
        &mut self._preview_options
    }

    pub fn sort_options_ref(&self) -> &SortOption {
        self.display_options_ref().sort_options_ref()
    }
    pub fn sort_options_mut(&mut self) -> &mut SortOption {
        self.display_options_mut().sort_options_mut()
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
