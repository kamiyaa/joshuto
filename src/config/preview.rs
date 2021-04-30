use serde_derive::Deserialize;
use std::collections::HashMap;

use super::{parse_to_config_file, ConfigStructure, Flattenable};

#[derive(Debug, Deserialize)]
pub struct JoshutoPreviewEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct JoshutoRawPreview {
    pub extension: Option<HashMap<String, JoshutoPreviewEntry>>,
    pub mimetype: Option<HashMap<String, JoshutoPreviewEntry>>,
}

impl std::default::Default for JoshutoRawPreview {
    fn default() -> Self {
        JoshutoRawPreview {
            extension: None,
            mimetype: None,
        }
    }
}

impl Flattenable<JoshutoPreview> for JoshutoRawPreview {
    fn flatten(self) -> JoshutoPreview {
        let extension = self.extension.unwrap_or_default();
        let mimetype = self.mimetype.unwrap_or_default();

        JoshutoPreview {
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

impl ConfigStructure for JoshutoPreview {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<JoshutoRawPreview, JoshutoPreview>(file_name)
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
