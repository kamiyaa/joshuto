use std::convert::From;

use ratatui::layout::Constraint;

use crate::config::raw::app::display::DisplayOptionRaw;

use super::{dirlist::DirListDisplayOptions, line_number::LineNumberStyle, tab::TabDisplayOption};

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
    pub _line_nums: LineNumberStyle,
    pub column_ratio: (usize, usize, usize),
    pub default_layout: [Constraint; 3],
    pub no_preview_layout: [Constraint; 3],
    pub default_tab_display_option: TabDisplayOption,
}

impl From<DisplayOptionRaw> for DisplayOption {
    fn from(raw: DisplayOptionRaw) -> Self {
        let mode = match raw.mode.as_str() {
            "hsplit" => DisplayMode::HSplit,
            _ => DisplayMode::Default,
        };

        let column_ratio = match raw.column_ratio {
            Some(s) if s.len() == 3 => (s[0], s[1], s[2]),
            Some(s) if s.len() == 2 => (0, s[0], s[1]),
            _ => default_column_ratio(),
        };

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

        let _line_nums = LineNumberStyle::from_str(raw.line_number_style.as_str())
            .unwrap_or(LineNumberStyle::None);

        Self {
            _mode: mode,
            _automatically_count_files: raw.automatically_count_files,
            _collapse_preview: raw.collapse_preview,
            _scroll_offset: raw.scroll_offset,
            _show_borders: raw.show_borders,
            _show_hidden: raw.show_hidden,
            _show_icons: raw.show_icons,
            _line_nums,

            column_ratio,
            default_layout,
            no_preview_layout,
            default_tab_display_option: TabDisplayOption {
                sort_options: raw.sort_options.into(),
                // todo: make default line mode configurable
                linemode: raw.linemode,
                ..Default::default()
            },
        }
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

    pub fn line_nums(&self) -> LineNumberStyle {
        self._line_nums
    }

    pub fn set_line_nums(&mut self, style: LineNumberStyle) {
        self._line_nums = style;
    }

    pub fn filter_func(
        &self,
    ) -> fn(&walkdir::DirEntry, &DisplayOption, &DirListDisplayOptions) -> bool {
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
            _line_nums: LineNumberStyle::None,
            default_layout,
            no_preview_layout,
            default_tab_display_option: TabDisplayOption::default(),
        }
    }
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn filter(
    entry: &walkdir::DirEntry,
    opt: &DisplayOption,
    dirlist_opts: &DirListDisplayOptions,
) -> bool {
    if !opt.show_hidden() && is_hidden(entry) {
        return false;
    }

    let file_name = match entry.file_name().to_str() {
        Some(s) => s,
        None => return false,
    };

    if !dirlist_opts.filter_context_ref().is_match(file_name) {
        return false;
    }

    true
}
