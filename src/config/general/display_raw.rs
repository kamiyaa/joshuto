use std::convert::From;

use ratatui::layout::Constraint;
use serde_derive::Deserialize;

use crate::config::option::{
    DisplayMode, DisplayOption, LineMode, LineNumberStyle, TabDisplayOption,
};

use super::sort_raw::SortOptionRaw;

pub const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

fn default_mode() -> String {
    "default".to_string()
}

const fn default_true() -> bool {
    true
}

const fn default_scroll_offset() -> usize {
    4
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisplayOptionRaw {
    #[serde(default = "default_mode")]
    pub mode: String,

    #[serde(default)]
    pub automatically_count_files: bool,

    #[serde(default = "default_true")]
    pub collapse_preview: bool,

    #[serde(default)]
    pub column_ratio: Option<Vec<usize>>,

    #[serde(default = "default_scroll_offset")]
    pub scroll_offset: usize,

    #[serde(default = "default_true")]
    pub show_borders: bool,

    #[serde(default)]
    pub show_hidden: bool,

    #[serde(default)]
    pub show_icons: bool,

    #[serde(default = "default_true")]
    pub tilde_in_titlebar: bool,

    #[serde(default, rename = "sort")]
    pub sort_options: SortOptionRaw,

    #[serde(default)]
    pub line_number_style: String,
}

impl std::default::Default for DisplayOptionRaw {
    fn default() -> Self {
        Self {
            mode: default_mode(),
            automatically_count_files: false,
            collapse_preview: true,
            column_ratio: None,
            scroll_offset: 4,
            show_borders: true,
            show_hidden: false,
            show_icons: false,
            sort_options: SortOptionRaw::default(),
            tilde_in_titlebar: true,
            line_number_style: "none".to_string(),
        }
    }
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
            _tilde_in_titlebar: raw.tilde_in_titlebar,
            _line_nums,

            column_ratio,
            default_layout,
            no_preview_layout,
            default_tab_display_option: TabDisplayOption {
                sort_options: raw.sort_options.into(),
                // todo: make default line mode configurable
                linemode: LineMode::Size,
                ..Default::default()
            },
        }
    }
}
