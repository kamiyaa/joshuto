use serde_derive::Deserialize;

use crate::util::sort_option::SortOption;
use crate::util::sort_type::{SortType, SortTypes};

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct SortRawOption {
    #[serde(default = "default_true")]
    pub directories_first: bool,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub reverse: bool,
    #[serde(default)]
    pub sort_method: Option<String>,
}

impl SortRawOption {
    pub fn into(self) -> SortOption {
        let sort_method = match self.sort_method.as_ref() {
            Some(s) => SortType::parse(s).unwrap_or(SortType::Natural),
            None => SortType::Natural,
        };

        let mut sort_methods = SortTypes::default();
        sort_methods.reorganize(sort_method);

        SortOption {
            directories_first: self.directories_first,
            case_sensitive: self.case_sensitive,
            reverse: self.reverse,
            sort_methods,
        }
    }
}

impl std::default::Default for SortRawOption {
    fn default() -> Self {
        Self {
            directories_first: default_true(),
            case_sensitive: bool::default(),
            reverse: bool::default(),
            sort_method: None,
        }
    }
}
