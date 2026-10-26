use serde::Deserialize;

use crate::config::utils::default_true;

/// TOML-deserializable form of [`SortOption`](crate::config::SortOption).
#[derive(Clone, Debug, Deserialize)]
pub struct FileSortConfigRaw {
    #[serde(default = "default_true")]
    pub directories_first: bool,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub reverse: bool,
    #[serde(default)]
    pub sort_method: Option<String>,
}

impl std::default::Default for FileSortConfigRaw {
    fn default() -> Self {
        Self {
            directories_first: true,
            case_sensitive: false,
            reverse: false,
            sort_method: None,
        }
    }
}
