use serde_derive::Deserialize;

use crate::util::sort;
use crate::util::sort::SortType;

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
    pub fn into(self) -> sort::SortOption {
        let sort_method = match self.sort_method.as_ref() {
            Some(s) => sort::SortType::parse(s).unwrap_or(sort::SortType::Natural),
            None => sort::SortType::Natural,
        };

        let mut sort_methods = std::collections::LinkedList::new();
        sort_methods.push_back(sort_method);
        sort_methods.push_back(SortType::Natural);
        sort_methods.push_back(SortType::Lexical);
        sort_methods.push_back(SortType::Size);
        sort_methods.push_back(SortType::Mtime);
        sort_methods.push_back(SortType::Ext);

        let sort_methods = sort::SortTypes { list: sort_methods };
        sort::SortOption {
            directories_first: self.directories_first,
            case_sensitive: self.case_sensitive,
            reverse: self.reverse,
            sort_method,
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
