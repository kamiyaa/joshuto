use serde::Deserialize;
use std::collections::HashMap;

/// Fallback icon glyphs used when no exact-name or extension match is found.
#[derive(Debug, Clone, Deserialize)]
pub struct DefaultIcons {
    #[serde(default)]
    pub file: String,
    #[serde(default)]
    pub directory: String,
}

/// TOML-deserializable form of [`AppIcons`](super::icon::AppIcons).
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
