use std::fs;

use tui::layout::Constraint;

use crate::config::option::SortOption;

#[derive(Clone, Copy, Debug)]
pub enum DisplayMode {
    Default,
    HSplit,
}

pub const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

/// Display options globally valid for Joshuto (for all tabs)
#[derive(Clone, Debug)]
pub struct DisplayOption {
    pub _mode: DisplayMode,
    pub _automatically_count_files: bool,
    pub _collapse_preview: bool,
    pub _scroll_offset: usize,
    pub _show_borders: bool,
    pub _show_hidden: bool,
    pub _show_icons: bool,
    pub _tilde_in_titlebar: bool,
    pub _line_nums: LineNumberStyle,
    pub column_ratio: (usize, usize, usize),
    pub default_layout: [Constraint; 3],
    pub no_preview_layout: [Constraint; 3],
    pub default_tab_display_option: TabDisplayOption,
}

/// Display options valid per JoshutoTab
#[derive(Clone, Debug)]
pub struct TabDisplayOption {
    pub _sort_options: SortOption,
    pub filter_string: String,
}

#[derive(Clone, Copy, Debug)]
pub enum LineNumberStyle {
    None,
    Relative,
    Absolute,
}

impl TabDisplayOption {
    pub fn sort_options_ref(&self) -> &SortOption {
        &self._sort_options
    }

    pub fn sort_options_mut(&mut self) -> &mut SortOption {
        &mut self._sort_options
    }

    pub fn set_filter_string(&mut self, pattern: &str) {
        self.filter_string = pattern.to_owned();
    }

    pub fn filter_string_ref(&self) -> &str {
        &self.filter_string
    }
}

impl DisplayOption {
    pub fn mode(&self) -> DisplayMode {
        self._mode
    }

    pub fn automatically_count_files(&self) -> bool {
        self._automatically_count_files
    }

    pub fn collapse_preview(&self) -> bool {
        self._collapse_preview
    }

    pub fn scroll_offset(&self) -> usize {
        self._scroll_offset
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

    pub fn set_show_hidden(&mut self, show_hidden: bool) {
        self._show_hidden = show_hidden;
    }

    pub fn tilde_in_titlebar(&self) -> bool {
        self._tilde_in_titlebar
    }

    pub fn line_nums(&self) -> LineNumberStyle {
        self._line_nums
    }

    pub fn set_line_nums(&mut self, style: LineNumberStyle) {
        self._line_nums = style;
    }

    pub fn filter_func(
        &self,
    ) -> fn(&Result<fs::DirEntry, std::io::Error>, &DisplayOption, &TabDisplayOption) -> bool {
        filter
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
            _mode: DisplayMode::Default,
            _automatically_count_files: false,
            _collapse_preview: true,
            column_ratio,
            _scroll_offset: 4,
            _show_borders: true,
            _show_hidden: false,
            _show_icons: false,
            _tilde_in_titlebar: true,
            _line_nums: LineNumberStyle::None,
            default_layout,
            no_preview_layout,
            default_tab_display_option: TabDisplayOption {
                _sort_options: SortOption::default(),
                filter_string: "".to_owned(),
            },
        }
    }
}

fn has_str(entry: &fs::DirEntry, pat: &str) -> bool {
    match entry.file_name().into_string().ok() {
        Some(s) => s
            .to_ascii_lowercase()
            .contains(pat.to_ascii_lowercase().as_str()),
        None => false,
    }
}

fn filter(
    result: &Result<fs::DirEntry, std::io::Error>,
    opt: &DisplayOption,
    tab_opts: &TabDisplayOption,
) -> bool {
    if opt.show_hidden() && tab_opts.filter_string_ref().is_empty() {
        true
    } else {
        match result {
            Err(_) => true,
            Ok(entry) => {
                if tab_opts.filter_string_ref().is_empty() {
                    let file_name = entry.file_name();
                    let lossy_string = file_name.as_os_str().to_string_lossy();
                    !lossy_string.starts_with('.')
                } else if opt.show_hidden() {
                    has_str(entry, tab_opts.filter_string_ref())
                } else {
                    let file_name = entry.file_name();
                    let lossy_string = file_name.as_os_str().to_string_lossy();
                    if !lossy_string.starts_with('.') {
                        has_str(entry, tab_opts.filter_string_ref())
                    } else {
                        false
                    }
                }
            }
        }
    }
}
