use serde::Deserialize;
use std::collections::HashMap;

use super::{parse_config_or_default, TomlConfigFile};

#[derive(Debug, Deserialize)]
pub struct JoshutoPreviewEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
struct JoshutoPreviewRaw {
    pub extension: Option<HashMap<String, JoshutoPreviewEntry>>,
    pub mimetype: Option<HashMap<String, JoshutoPreviewEntry>>,
}

impl From<JoshutoPreviewRaw> for JoshutoPreview {
    fn from(crude: JoshutoPreviewRaw) -> Self {
        let extension = crude.extension.unwrap_or_default();
        let mimetype = crude.mimetype.unwrap_or_default();

        Self {
            extension,
            mimetype,
        }
    }
}

#[derive(Debug, Default)]
pub struct JoshutoPreview {
    pub extension: HashMap<String, JoshutoPreviewEntry>,
    pub mimetype: HashMap<String, JoshutoPreviewEntry>,
}

impl TomlConfigFile for JoshutoPreview {
    fn get_config(file_name: &str) -> Self {
        parse_config_or_default::<JoshutoPreviewRaw, JoshutoPreview>(file_name)
    }
}
