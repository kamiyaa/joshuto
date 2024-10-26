use std::convert::From;

use ratatui::layout::Constraint;

use crate::{
    config::display_raw::DisplayOptionRaw, fs::DirListDisplayOptions, tab::TabDisplayOption,
};

use super::line_mode::LineNumberStyle;

#[derive(Clone, Copy, Debug)]
pub enum DisplayMode {
    Default,
    Minimal,
    HSplit,
}

pub const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

/// Display options globally valid for Joshuto (for all tabs)
#[derive(Clone, Debug)]
pub struct DisplayOption {
    pub mode: DisplayMode,
    pub automatically_count_files: bool,
    pub collapse_preview: bool,
    pub scroll_offset: usize,
    pub show_borders: bool,
    pub show_hidden: bool,
    pub show_icons: bool,
    pub line_number_style: LineNumberStyle,
    pub default_layout: [Constraint; 3],
    pub no_preview_layout: [Constraint; 3],
    pub default_tab_display_option: TabDisplayOption,
}

impl From<DisplayOptionRaw> for DisplayOption {
    fn from(raw: DisplayOptionRaw) -> Self {
        let mode = match raw.mode.as_str() {
            "hsplit" => DisplayMode::HSplit,
            "minimal" => DisplayMode::Minimal,
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

        Self {
            mode,
            automatically_count_files: raw.automatically_count_files,
            collapse_preview: raw.collapse_preview,
            scroll_offset: raw.scroll_offset,
            show_borders: raw.show_borders,
            show_hidden: raw.show_hidden,
            show_icons: raw.show_icons,
            line_number_style: raw.line_number_style,

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
            mode: DisplayMode::Default,
            automatically_count_files: false,
            collapse_preview: true,
            scroll_offset: 4,
            show_borders: true,
            show_hidden: false,
            show_icons: false,
            line_number_style: LineNumberStyle::None,
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
    if !opt.show_hidden && is_hidden(entry) {
        return false;
    }

    let file_name = match entry.file_name().to_str() {
        Some(s) => s,
        None => return false,
    };

    if !dirlist_opts.filter_state_ref().is_match(file_name) {
        return false;
    }

    true
}
