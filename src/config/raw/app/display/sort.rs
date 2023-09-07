use serde::Deserialize;

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct SortOptionRaw {
    #[serde(default = "default_true")]
    pub directories_first: bool,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub reverse: bool,
    #[serde(default)]
    pub sort_method: Option<String>,
}

impl std::default::Default for SortOptionRaw {
    fn default() -> Self {
        Self {
            directories_first: default_true(),
            case_sensitive: bool::default(),
            reverse: bool::default(),
            sort_method: None,
        }
    }
}
