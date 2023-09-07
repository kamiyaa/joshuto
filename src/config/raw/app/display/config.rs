use serde::Deserialize;

use crate::config::clean::app::display::line_mode::LineMode;

use super::sort::SortOptionRaw;

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

    #[serde(default)]
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
            tilde_in_titlebar: true,
            line_number_style: "none".to_string(),
            linemode: LineMode::default(),
        }
    }
}
