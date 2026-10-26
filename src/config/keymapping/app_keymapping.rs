use std::convert::From;

use crate::config::keymapping::app_keymapping_raw::AppKeyMappingRaw;
use crate::config::keymapping::utils::command_keymaps_vec_to_map;
use crate::config::KeyMapping;
use crate::config::{AppPersistentConfig, ConfigType};
use crate::constants::config::DEFAULT_KEYMAP_CONFIG;
use crate::error::{AppError, AppErrorCode, AppResult};

/// The full set of keybindings for every UI mode, loaded from `keymap.toml`.
#[derive(Debug)]
pub struct AppKeybindConfig {
    pub default_view: KeyMapping,
    pub task_view: KeyMapping,
    pub help_view: KeyMapping,
}

impl AppKeybindConfig {
    /// Creates an empty keymap with no bindings in any view.
    pub fn new() -> Self {
        Self {
            default_view: KeyMapping::new(),
            task_view: KeyMapping::new(),
            help_view: KeyMapping::new(),
        }
    }

    /// Parses the built-in default `keymap.toml`, returning an error only if it's malformed.
    pub fn try_default() -> AppResult<Self> {
        let value: AppKeyMappingRaw = toml::from_str(DEFAULT_KEYMAP_CONFIG).map_err(|err| {
            let error_msg = "Failed to parse default config";
            tracing::error!(?err, "{error_msg}");
            AppError::new(AppErrorCode::DefaultConfigParse, error_msg.to_string())
        })?;
        let keymapping: Self = Self::from(value);
        Ok(keymapping)
    }
}

impl From<AppKeyMappingRaw> for AppKeybindConfig {
    fn from(raw: AppKeyMappingRaw) -> Self {
        let mut keymaps = Self::new();
        keymaps.default_view = command_keymaps_vec_to_map(&raw.default_view.keymap);
        keymaps.task_view = command_keymaps_vec_to_map(&raw.task_view.keymap);
        keymaps.help_view = command_keymaps_vec_to_map(&raw.help_view.keymap);
        keymaps
    }
}

impl AppPersistentConfig for AppKeybindConfig {
    type Raw = AppKeyMappingRaw;

    fn get_type() -> ConfigType {
        ConfigType::Keymap
    }
}

impl std::default::Default for AppKeybindConfig {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        AppKeybindConfig::try_default().unwrap()
    }
}
