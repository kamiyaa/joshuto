use serde::Deserialize;

use crate::{
    types::option::line_mode::{LineMode, LineNumberStyle},
    utils::serde::{default_mode, default_scroll_offset, default_true, deserialize_line_mode},
};

use super::sort_option_raw::SortOptionRaw;

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
    #[serde(default, rename = "sort")]
    pub sort_options: SortOptionRaw,
    #[serde(default)]
    pub line_number_style: LineNumberStyle,
    #[serde(default, deserialize_with = "deserialize_line_mode")]
    pub linemode: LineMode,
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
            line_number_style: LineNumberStyle::default(),
            linemode: LineMode::default(),
        }
    }
}
