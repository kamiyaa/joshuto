use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    config::{utils::search_directories, ConfigType},
    error::{AppError, AppErrorCode, AppResult},
    global,
    tasks::preview::preview_entry::FileEntryPreviewEntry,
};

/// Per-extension and per-mimetype external preview program overrides.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FileEntryPreview {
    #[serde(default)]
    pub extension: HashMap<String, FileEntryPreviewEntry>,
    #[serde(default)]
    pub mimetype: HashMap<String, FileEntryPreviewEntry>,
}

impl FileEntryPreview {
    /// Parses a `FileEntryPreview` from TOML source text.
    pub fn from_toml_str(s: &str) -> AppResult<Self> {
        let res = toml::from_str(s)?;
        Ok(res)
    }

    /// Loads and parses this config from the config directory hierarchy.
    pub fn read_config() -> AppResult<Self> {
        let file_path =
            search_directories(&global::CONFIG_HIERARCHY, ConfigType::Preview.as_filename())
                .ok_or_else(|| {
                    AppError::new(AppErrorCode::ConfigRead, "Cannot find config".to_string())
                })?;
        let file_contents = std::fs::read_to_string(file_path)?;
        Self::from_toml_str(&file_contents)
    }

    /// Loads this config, falling back to the default (empty) config if not found or invalid.
    pub fn read_config_or_default() -> Self {
        Self::read_config().unwrap_or_default()
    }
}
