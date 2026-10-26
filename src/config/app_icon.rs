use std::collections::HashMap;

use crate::{
    config::{app_icon_raw::IconsRaw, AppPersistentConfig, ConfigType},
    constants::config::DEFAULT_ICON_CONFIG,
    error::{AppError, AppErrorCode, AppResult},
};

/// Icon glyphs used in the file list, keyed by exact name/extension with configurable fallbacks.
#[derive(Debug)]
pub struct AppIcons {
    pub directory_exact: HashMap<String, String>,
    pub file_exact: HashMap<String, String>,
    pub ext: HashMap<String, String>,
    pub default_file: String,
    pub default_dir: String,
}

impl AppIcons {
    /// Parses the built-in default `icons.toml`, returning an error only if it's malformed.
    pub fn default_icons() -> AppResult<Self> {
        let icons: IconsRaw = toml::from_str(DEFAULT_ICON_CONFIG).map_err(|err| {
            let error_msg = "Failed to parse default config";
            tracing::error!(?err, "{error_msg}");
            AppError::new(AppErrorCode::DefaultConfigParse, error_msg.to_string())
        })?;
        Ok(Self::from(icons))
    }
}

impl std::default::Default for AppIcons {
    fn default() -> Self {
        Self::default_icons().unwrap()
    }
}

impl AppPersistentConfig for AppIcons {
    type Raw = IconsRaw;

    fn get_type() -> ConfigType {
        ConfigType::Icons
    }
}

impl From<IconsRaw> for AppIcons {
    fn from(raw: IconsRaw) -> Self {
        AppIcons {
            directory_exact: raw.directory_exact,
            file_exact: raw.file_exact,
            ext: raw.ext,
            default_file: raw.defaults.file,
            default_dir: raw.defaults.directory,
        }
    }
}
