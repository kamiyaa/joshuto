use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct DefaultIcons {
    #[serde(default)]
    pub file: String,
    #[serde(default)]
    pub directory: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IconsRaw {
    #[serde(default)]
    pub directory_exact: HashMap<String, String>,
    #[serde(default)]
    pub file_exact: HashMap<String, String>,
    #[serde(default)]
    pub ext: HashMap<String, String>,
    #[serde(default)]
    pub defaults: DefaultIcons,
}

impl std::default::Default for DefaultIcons {
    fn default() -> Self {
        Self {
            file: "".to_string(),
            directory: "".to_string(),
        }
    }
}
