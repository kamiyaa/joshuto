use serde_derive::Deserialize;

use tui::layout::Constraint;

use super::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::util::sort;

use crate::CONFIG_FILE;

const fn default_true() -> bool {
    true
}
const fn default_scroll_offset() -> usize {
    6
}
const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}
const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

#[derive(Clone, Debug, Deserialize)]
struct SortRawOption {
    #[serde(default)]
    show_icons: bool,
    #[serde(default)]
    show_hidden: bool,
    #[serde(default = "default_true")]
    directories_first: bool,
    #[serde(default)]
    case_sensitive: bool,
    #[serde(default)]
    reverse: bool,
}

impl SortRawOption {
    pub fn into_sort_option(self, sort_method: sort::SortType) -> sort::SortOption {
        sort::SortOption {
            show_icons: self.show_icons,
            show_hidden: self.show_hidden,
            directories_first: self.directories_first,
            case_sensitive: self.case_sensitive,
            reverse: self.reverse,
            sort_method,
        }
    }
}

impl std::default::Default for SortRawOption {
    fn default() -> Self {
        Self {
            show_icons: bool::default(),
            show_hidden: bool::default(),
            directories_first: default_true(),
            case_sensitive: bool::default(),
            reverse: bool::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoRawConfig {
    #[serde(default = "default_true")]
    collapse_preview: bool,
    #[serde(default = "default_scroll_offset")]
    scroll_offset: usize,
    #[serde(default = "default_true")]
    show_borders: bool,
    #[serde(default = "default_true")]
    show_preview: bool,
    #[serde(default = "default_true")]
    tilde_in_titlebar: bool,
    #[serde(default = "default_true")]
    use_trash: bool,
    #[serde(default)]
    xdg_open: bool,
    #[serde(default = "default_max_preview_size")]
    max_preview_size: u64,
    column_ratio: Option<[usize; 3]>,
    sort_method: Option<String>,
    #[serde(default)]
    sort_option: SortRawOption,
}

impl Flattenable<JoshutoConfig> for JoshutoRawConfig {
    fn flatten(self) -> JoshutoConfig {
        let column_ratio = match self.column_ratio {
            Some(s) => (s[0], s[1], s[2]),
            _ => default_column_ratio(),
        };

        let sort_method = match self.sort_method {
            Some(s) => match sort::SortType::parse(s.as_str()) {
                Some(s) => s,
                None => sort::SortType::Natural,
            },
            None => sort::SortType::Natural,
        };
        let sort_option = self.sort_option.into_sort_option(sort_method);

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

        JoshutoConfig {
            collapse_preview: self.collapse_preview,
            max_preview_size: self.max_preview_size,
            scroll_offset: self.scroll_offset,
            show_borders: self.show_borders,
            show_preview: self.show_preview,
            tilde_in_titlebar: self.tilde_in_titlebar,
            use_trash: self.use_trash,
            xdg_open: self.xdg_open,
            column_ratio,
            sort_option,
            default_layout,
            no_preview_layout,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JoshutoConfig {
    pub collapse_preview: bool,
    pub max_preview_size: u64,
    pub show_preview: bool,
    pub show_borders: bool,
    pub scroll_offset: usize,
    pub tilde_in_titlebar: bool,
    pub use_trash: bool,
    pub xdg_open: bool,
    pub sort_option: sort::SortOption,
    pub column_ratio: (usize, usize, usize),
    pub default_layout: [Constraint; 3],
    pub no_preview_layout: [Constraint; 3],
}

impl ConfigStructure for JoshutoConfig {
    fn get_config() -> Self {
        parse_to_config_file::<JoshutoRawConfig, JoshutoConfig>(CONFIG_FILE)
            .unwrap_or_else(Self::default)
    }
}

impl std::default::Default for JoshutoConfig {
    fn default() -> Self {
        let sort_option = sort::SortOption::default();

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
            collapse_preview: true,
            max_preview_size: default_max_preview_size(),
            show_preview: true,
            show_borders: false,
            scroll_offset: default_scroll_offset(),
            tilde_in_titlebar: true,
            use_trash: true,
            xdg_open: false,
            sort_option,
            column_ratio,
            default_layout,
            no_preview_layout,
        }
    }
}
