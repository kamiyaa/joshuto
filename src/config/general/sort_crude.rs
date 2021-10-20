use std::convert::From;

use serde_derive::Deserialize;

use crate::config::option::{SortOption, SortType, SortTypes};

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct SortOptionCrude {
    #[serde(default = "default_true")]
    pub directories_first: bool,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub reverse: bool,
    #[serde(default)]
    pub sort_method: Option<String>,
}

impl std::default::Default for SortOptionCrude {
    fn default() -> Self {
        Self {
            directories_first: default_true(),
            case_sensitive: bool::default(),
            reverse: bool::default(),
            sort_method: None,
        }
    }
}

impl From<SortOptionCrude> for SortOption {
    fn from(crude: SortOptionCrude) -> Self {
        let sort_method = match crude.sort_method.as_ref() {
            Some(s) => SortType::parse(s).unwrap_or(SortType::Natural),
            None => SortType::Natural,
        };

        let mut sort_methods = SortTypes::default();
        sort_methods.reorganize(sort_method);

        Self {
            directories_first: crude.directories_first,
            case_sensitive: crude.case_sensitive,
            reverse: crude.reverse,
            sort_methods,
        }
    }
}
