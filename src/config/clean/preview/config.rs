use std::collections::HashMap;

use serde::Deserialize;

use crate::config::{parse_config_or_default, raw::preview::FileEntryPreviewRaw, TomlConfigFile};

#[derive(Debug, Deserialize)]
pub struct FileEntryPreviewEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Default)]
pub struct FileEntryPreview {
    pub extension: HashMap<String, FileEntryPreviewEntry>,
    pub mimetype: HashMap<String, FileEntryPreviewEntry>,
}

impl TomlConfigFile for FileEntryPreview {
    fn get_config(file_name: &str) -> Self {
        parse_config_or_default::<FileEntryPreviewRaw, FileEntryPreview>(file_name)
    }
}

impl From<FileEntryPreviewRaw> for FileEntryPreview {
    fn from(raw: FileEntryPreviewRaw) -> Self {
        let extension = raw.extension.unwrap_or_default();
        let mimetype = raw.mimetype.unwrap_or_default();

        Self {
            extension,
            mimetype,
        }
    }
}
