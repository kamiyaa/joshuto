use serde_derive::Deserialize;
use tui::layout::Constraint;

use crate::config::Flattenable;
use crate::util::display::{default_column_ratio, DisplayOption};

use super::SortRawOption;

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisplayRawOption {
    #[serde(default)]
    automatically_count_files: bool,

    #[serde(default = "default_true")]
    collapse_preview: bool,

    #[serde(default)]
    column_ratio: Option<[usize; 3]>,

    #[serde(default = "default_true")]
    show_borders: bool,

    #[serde(default)]
    show_hidden: bool,

    #[serde(default)]
    show_icons: bool,

    #[serde(default = "default_true")]
    show_preview: bool,

    #[serde(default = "default_true")]
    tilde_in_titlebar: bool,

    #[serde(default, rename = "sort")]
    sort_options: SortRawOption,
}

impl Flattenable<DisplayOption> for DisplayRawOption {
    fn flatten(self) -> DisplayOption {
        let column_ratio = match self.column_ratio {
            Some(s) => (s[0], s[1], s[2]),
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

        DisplayOption {
            _automatically_count_files: self.automatically_count_files,
            _collapse_preview: self.collapse_preview,
            column_ratio,
            _show_borders: self.show_borders,
            _show_hidden: self.show_hidden,
            _show_icons: self.show_icons,
            _show_preview: self.show_preview,
            _sort_options: self.sort_options.into(),
            _tilde_in_titlebar: self.tilde_in_titlebar,
            default_layout,
            no_preview_layout,
        }
    }
}

impl std::default::Default for DisplayRawOption {
    fn default() -> Self {
        Self {
            automatically_count_files: false,
            collapse_preview: true,
            column_ratio: None,
            show_borders: true,
            show_hidden: false,
            show_icons: false,
            show_preview: true,
            sort_options: SortRawOption::default(),
            tilde_in_titlebar: true,
        }
    }
}
