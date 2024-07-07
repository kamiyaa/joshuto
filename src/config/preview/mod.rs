pub mod preview_option_raw;

use std::collections::HashMap;

use crate::{
    error::{AppError, AppErrorKind, AppResult},
    preview::preview_entry::FileEntryPreviewEntry,
    traits::config::search_config_directories,
    types::config_type::ConfigType,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FileEntryPreview {
    #[serde(default)]
    pub extension: HashMap<String, FileEntryPreviewEntry>,
    #[serde(default)]
    pub mimetype: HashMap<String, FileEntryPreviewEntry>,
}

impl FileEntryPreview {
    pub fn from_toml_str(s: &str) -> AppResult<Self> {
        let res = toml::from_str(s)?;
        Ok(res)
    }

    pub fn get_config() -> AppResult<Self> {
        let file_path = search_config_directories(ConfigType::Preview.as_filename())
            .ok_or_else(|| AppError::new(AppErrorKind::Config, "Cannot find config".to_string()))?;
        let file_contents = std::fs::read_to_string(file_path)?;
        Self::from_toml_str(&file_contents)
    }

    pub fn get_config_or_default() -> Self {
        Self::get_config().unwrap_or_default()
    }
}
