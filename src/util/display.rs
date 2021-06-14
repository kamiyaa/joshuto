use std::fs;

use tui::layout::Constraint;

use crate::util::sort;

pub const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

#[derive(Clone, Debug)]
pub struct DisplayOption {
    pub _collapse_preview: bool,
    pub column_ratio: (usize, usize, usize),
    pub _show_borders: bool,
    pub _show_hidden: bool,
    pub _show_icons: bool,
    pub _show_preview: bool,
    pub _sort_options: sort::SortOption,
    pub _tilde_in_titlebar: bool,
    pub default_layout: [Constraint; 3],
    pub no_preview_layout: [Constraint; 3],
}

impl DisplayOption {
    pub fn collapse_preview(&self) -> bool {
        self._collapse_preview
    }

    pub fn show_borders(&self) -> bool {
        self._show_borders
    }

    pub fn show_hidden(&self) -> bool {
        self._show_hidden
    }

    pub fn show_icons(&self) -> bool {
        self._show_icons
    }

    #[allow(dead_code)]
    pub fn show_preview(&self) -> bool {
        self._show_preview
    }

    pub fn set_show_hidden(&mut self, show_hidden: bool) {
        self._show_hidden = show_hidden;
    }

    pub fn sort_options_ref(&self) -> &sort::SortOption {
        &self._sort_options
    }

    pub fn sort_options_mut(&mut self) -> &mut sort::SortOption {
        &mut self._sort_options
    }

    pub fn tilde_in_titlebar(&self) -> bool {
        self._tilde_in_titlebar
    }

    pub fn filter_func(&self) -> fn(&Result<fs::DirEntry, std::io::Error>) -> bool {
        if self.show_hidden() {
            no_filter
        } else {
            filter_hidden
        }
    }
}

impl std::default::Default for DisplayOption {
    fn default() -> Self {
        let column_ratio = default_column_ratio();

        let total = (column_ratio.0 + column_ratio.1 + column_ratio.2) as u32;
        let default_layout = [
            Constraint::Ratio(column_ratio.0 as u32, total),
            Constraint::Ratio(column_ratio.1 as u32, total),
            Constraint::Ratio(column_ratio.2 as u32, total),
        ];
        let no_preview_layout = [
            Constraint::Ratio(column_ratio.0 as u32, total),
            Constraint::Ratio(column_ratio.1 as u32 + column_ratio.2 as u32, total),
            Constraint::Ratio(0, total),
        ];

        Self {
            _collapse_preview: true,
            column_ratio,
            _show_borders: true,
            _show_hidden: false,
            _show_icons: false,
            _show_preview: true,
            _sort_options: sort::SortOption::default(),
            _tilde_in_titlebar: true,
            default_layout,
            no_preview_layout,
        }
    }
}

const fn no_filter(_: &Result<fs::DirEntry, std::io::Error>) -> bool {
    true
}

fn filter_hidden(result: &Result<fs::DirEntry, std::io::Error>) -> bool {
    match result {
        Err(_) => true,
        Ok(entry) => {
            let file_name = entry.file_name();
            let lossy_string = file_name.as_os_str().to_string_lossy();
            !lossy_string.starts_with('.')
        }
    }
}
