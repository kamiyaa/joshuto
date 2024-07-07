use std::collections::HashMap;

use serde::Deserialize;

use crate::{traits::config::TomlConfigFile, types::config_type::ConfigType};

use super::preview_raw::FileEntryPreviewRaw;

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
    type Raw = FileEntryPreviewRaw;

    fn get_type() -> ConfigType {
        ConfigType::Preview
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
