use serde_derive::Deserialize;
use std::collections::HashMap;

use crate::config::{parse_config_file, Flattenable};
use crate::PREVIEW_FILE;

#[derive(Debug, Deserialize)]
pub struct JoshutoPreviewEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawPreview {
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

impl JoshutoPreview {
    pub fn new() -> Self {
        JoshutoPreview {
            extension: HashMap::new(),
            mimetype: HashMap::new(),
        }
    }

    pub fn get_config() -> JoshutoPreview {
        parse_config_file::<JoshutoRawPreview, JoshutoPreview>(PREVIEW_FILE)
            .unwrap_or_else(JoshutoPreview::new)
    }
}
