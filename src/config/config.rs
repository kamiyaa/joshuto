use serde_derive::Deserialize;

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
    #[serde(default = "default_scroll_offset")]
    scroll_offset: usize,
    #[serde(default = "default_true")]
    tilde_in_titlebar: bool,
    #[serde(default = "default_true")]
    show_preview: bool,
    #[serde(default = "default_true")]
    collapse_preview: bool,
    #[serde(default)]
    xdg_open: bool,
    #[serde(default = "default_true")]
    use_trash: bool,
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

        JoshutoConfig {
            scroll_offset: self.scroll_offset,
            tilde_in_titlebar: self.tilde_in_titlebar,
            show_preview: self.show_preview,
            collapse_preview: self.collapse_preview,
            xdg_open: self.xdg_open,
            use_trash: self.use_trash,
            max_preview_size: self.max_preview_size,
            column_ratio,
            sort_option,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JoshutoConfig {
    pub scroll_offset: usize,
    pub tilde_in_titlebar: bool,
    pub show_preview: bool,
    pub collapse_preview: bool,
    pub xdg_open: bool,
    pub use_trash: bool,
    pub max_preview_size: u64,
    pub sort_option: sort::SortOption,
    pub column_ratio: (usize, usize, usize),
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

        Self {
            scroll_offset: default_scroll_offset(),
            tilde_in_titlebar: true,
            show_preview: true,
            collapse_preview: true,
            xdg_open: false,
            use_trash: true,
            max_preview_size: default_max_preview_size(),
            sort_option,
            column_ratio: default_column_ratio(),
        }
    }
}
