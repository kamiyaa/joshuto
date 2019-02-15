extern crate toml;
extern crate xdg;

use std::collections::HashMap;

use config::{parse_config_file, Flattenable};
use PREVIEW_FILE;

#[derive(Debug, Deserialize)]
pub struct JoshutoPreviewEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawPreview {
    pub mimetype: Option<HashMap<String, JoshutoPreviewEntry>>,
    pub extension: Option<HashMap<String, JoshutoPreviewEntry>>,
}

impl JoshutoRawPreview {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JoshutoRawPreview {
            mimetype: None,
            extension: None,
        }
    }
}

impl Flattenable<JoshutoPreview> for JoshutoRawPreview {
    fn flatten(self) -> JoshutoPreview {
        let mimetype = self.mimetype.unwrap_or(HashMap::new());
        let extension = self.extension.unwrap_or(HashMap::new());

        JoshutoPreview {
            mimetype,
            extension,
        }
    }
}

#[derive(Debug)]
pub struct JoshutoPreview {
    pub mimetype: HashMap<String, JoshutoPreviewEntry>,
    pub extension: HashMap<String, JoshutoPreviewEntry>,
}

impl JoshutoPreview {
    pub fn new() -> Self {
        JoshutoPreview {
            mimetype: HashMap::new(),
            extension: HashMap::new(),
        }
    }

    pub fn get_config() -> JoshutoPreview {
        parse_config_file::<JoshutoRawPreview, JoshutoPreview>(PREVIEW_FILE)
            .unwrap_or_else(|| JoshutoPreview::new())
    }
}
