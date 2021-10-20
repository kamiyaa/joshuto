use serde_derive::Deserialize;
use std::collections::HashMap;

use super::{parse_to_config_file, TomlConfigFile};

#[derive(Debug, Deserialize)]
pub struct JoshutoPreviewEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct JoshutoPreviewCrude {
    pub extension: Option<HashMap<String, JoshutoPreviewEntry>>,
    pub mimetype: Option<HashMap<String, JoshutoPreviewEntry>>,
}

impl std::default::Default for JoshutoPreviewCrude {
    fn default() -> Self {
        Self {
            extension: None,
            mimetype: None,
        }
    }
}

impl From<JoshutoPreviewCrude> for JoshutoPreview {
    fn from(crude: JoshutoPreviewCrude) -> Self {
        let extension = crude.extension.unwrap_or_default();
        let mimetype = crude.mimetype.unwrap_or_default();

        Self {
            extension,
            mimetype,
        }
    }
}

#[derive(Debug)]
pub struct JoshutoPreview {
    pub extension: HashMap<String, JoshutoPreviewEntry>,
    pub mimetype: HashMap<String, JoshutoPreviewEntry>,
}

impl TomlConfigFile for JoshutoPreview {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<JoshutoPreviewCrude, JoshutoPreview>(file_name)
            .unwrap_or_else(JoshutoPreview::default)
    }
}

impl std::default::Default for JoshutoPreview {
    fn default() -> Self {
        JoshutoPreview {
            extension: HashMap::new(),
            mimetype: HashMap::new(),
        }
    }
}
