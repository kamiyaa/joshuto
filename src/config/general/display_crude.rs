use std::convert::From;

use serde_derive::Deserialize;
use tui::layout::Constraint;

use crate::config::option::{DisplayOption, LineNumberStyle};

use super::sort_crude::SortOptionCrude;

pub const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

const fn default_true() -> bool {
    true
}

const fn default_scroll_offset() -> usize {
    4
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisplayOptionCrude {
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
    pub show_preview: bool,

    #[serde(default = "default_true")]
    pub tilde_in_titlebar: bool,

    #[serde(default, rename = "sort")]
    pub sort_options: SortOptionCrude,

    #[serde(default)]
    pub line_number_style: String,
}

impl std::default::Default for DisplayOptionCrude {
    fn default() -> Self {
        Self {
            automatically_count_files: false,
            collapse_preview: true,
            column_ratio: None,
            scroll_offset: 4,
            show_borders: true,
            show_hidden: false,
            show_icons: false,
            show_preview: true,
            sort_options: SortOptionCrude::default(),
            tilde_in_titlebar: true,
            line_number_style: "none".to_string(),
        }
    }
}

impl From<DisplayOptionCrude> for DisplayOption {
    fn from(crude: DisplayOptionCrude) -> Self {
        let column_ratio = match crude.column_ratio {
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

        let _line_nums = match crude.line_number_style.as_ref() {
            "absolute" => LineNumberStyle::Absolute,
            "relative" => LineNumberStyle::Relative,
            _ => LineNumberStyle::None,
        };

        Self {
            _automatically_count_files: crude.automatically_count_files,
            _collapse_preview: crude.collapse_preview,
            _scroll_offset: crude.scroll_offset,
            _show_borders: crude.show_borders,
            _show_hidden: crude.show_hidden,
            _show_icons: crude.show_icons,
            _show_preview: crude.show_preview,
            _sort_options: crude.sort_options.into(),
            _tilde_in_titlebar: crude.tilde_in_titlebar,
            _line_nums,

            column_ratio,
            default_layout,
            no_preview_layout,
        }
    }
}
