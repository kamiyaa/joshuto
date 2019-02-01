extern crate toml;
extern crate whoami;
extern crate xdg;

use joshuto;
use joshuto::config::{parse_config, Flattenable};
use joshuto::sort;

#[derive(Clone, Debug, Deserialize)]
pub struct SortRawOption {
    pub show_hidden: Option<bool>,
    pub directories_first: Option<bool>,
    pub case_sensitive: Option<bool>,
    pub reverse: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoRawConfig {
    scroll_offset: Option<usize>,
    tilde_in_titlebar: Option<bool>,
    sort_type: Option<String>,
    sort_option: Option<SortRawOption>,
    column_ratio: Option<[usize; 3]>,
}

impl JoshutoRawConfig {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JoshutoRawConfig {
            scroll_offset: Some(8),
            tilde_in_titlebar: Some(true),
            sort_type: Some(String::from("natural")),
            sort_option: None,
            column_ratio: Some([1, 3, 4]),
        }
    }
}

impl Flattenable<JoshutoConfig> for JoshutoRawConfig {
    fn flatten(self) -> JoshutoConfig {
        let column_ratio = match self.column_ratio {
            Some(s) => (s[0], s[1], s[2]),
            None => (1, 3, 4),
        };

        let scroll_offset: usize = self.scroll_offset.unwrap_or(6);
        let tilde_in_titlebar: bool = self.tilde_in_titlebar.unwrap_or(true);

        let show_hidden: bool;
        let case_sensitive: bool;
        let reverse: bool;
        let directories_first: bool;

        match self.sort_option {
            Some(s) => {
                show_hidden = s.show_hidden.unwrap_or(false);
                case_sensitive = s.case_sensitive.unwrap_or(false);
                reverse = s.reverse.unwrap_or(false);
                directories_first = s.directories_first.unwrap_or(true);
            }
            None => {
                show_hidden = false;
                case_sensitive = false;
                reverse = false;
                directories_first = true;
            }
        }

        let sort_option = sort::SortOption {
            show_hidden,
            directories_first,
            case_sensitive,
            reverse,
        };

        let sort_type: sort::SortType = match self.sort_type {
            Some(s) => match s.as_str() {
                "natural" => sort::SortType::SortNatural(sort_option),
                "mtime" => sort::SortType::SortMtime(sort_option),
                _ => sort::SortType::SortNatural(sort_option),
            },
            _ => sort::SortType::SortNatural(sort_option),
        };

        JoshutoConfig {
            scroll_offset,
            tilde_in_titlebar,
            sort_type,
            column_ratio,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JoshutoConfig {
    pub scroll_offset: usize,
    pub tilde_in_titlebar: bool,
    pub sort_type: joshuto::sort::SortType,
    pub column_ratio: (usize, usize, usize),
}

impl JoshutoConfig {
    pub fn new() -> Self {
        let sort_option = sort::SortOption {
            show_hidden: false,
            directories_first: true,
            case_sensitive: false,
            reverse: false,
        };
        let sort_type = sort::SortType::SortNatural(sort_option);

        JoshutoConfig {
            scroll_offset: 6,
            tilde_in_titlebar: true,
            sort_type,
            column_ratio: (1, 3, 4),
        }
    }

    pub fn get_config() -> JoshutoConfig {
        parse_config::<JoshutoRawConfig, JoshutoConfig>(::CONFIG_FILE)
            .unwrap_or_else(|| JoshutoConfig::new())
    }
}
