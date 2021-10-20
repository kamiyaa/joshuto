use crate::config::option::{DisplayOption, PreviewOption, SortOption, TabOption};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub use_trash: bool,
    pub xdg_open: bool,
    pub _display_options: DisplayOption,
    pub _preview_options: PreviewOption,
    pub _tab_options: TabOption,
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
        Self {
            use_trash: true,
            xdg_open: false,
            _display_options: DisplayOption::default(),
            _preview_options: PreviewOption::default(),
            _tab_options: TabOption::default(),
        }
    }
}
