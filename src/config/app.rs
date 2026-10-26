use std::{collections::HashMap, path::PathBuf};

use crate::{
    config::{
        app_raw::AppConfigRaw, AppPersistentConfig, AppTabOption, ConfigType, CustomCommand,
        DisplayOption, PreviewOption, SearchOption,
    },
    constants::config::DEFAULT_APP_CONFIG,
    error::{AppError, AppErrorCode, AppResult},
};

/// Top-level application settings, loaded from `joshuto.toml`.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub use_trash: bool,
    pub xdg_open: bool,
    pub xdg_open_fork: bool,
    pub case_insensitive_ext: bool,
    pub watch_files: bool,
    pub custom_commands: Vec<CustomCommand>,
    pub focus_on_create: bool,
    pub mouse_support: bool,
    pub cmd_aliases: HashMap<String, String>,
    pub zoxide_update: bool,
    pub display_options: DisplayOption,
    pub preview_options: PreviewOption,
    pub search_options: SearchOption,
    pub tab_options: AppTabOption,
    pub shell: PathBuf,
}

impl AppConfig {
    /// Parses the built-in default `joshuto.toml`, returning an error only if it's malformed.
    pub fn try_default() -> AppResult<Self> {
        let value: AppConfigRaw = toml::from_str(DEFAULT_APP_CONFIG).map_err(|err| {
            let error_msg = "Failed to parse default config";
            tracing::error!(?err, "{error_msg}");
            AppError::new(AppErrorCode::DefaultConfigParse, error_msg.to_string())
        })?;
        Ok(Self::from(value))
    }
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        Self::try_default().unwrap()
    }
}

impl From<AppConfigRaw> for AppConfig {
    fn from(value: AppConfigRaw) -> Self {
        Self {
            use_trash: value.use_trash,
            xdg_open: value.xdg_open,
            xdg_open_fork: value.xdg_open_fork,
            case_insensitive_ext: value.case_insensitive_ext,
            watch_files: value.watch_files,
            cmd_aliases: value.cmd_aliases,
            focus_on_create: value.focus_on_create,
            mouse_support: value.mouse_support,
            zoxide_update: value.zoxide_update,
            display_options: DisplayOption::from(value.display_options),
            preview_options: PreviewOption::from(value.preview_options),
            search_options: value.search_options,
            tab_options: value.tab_options,
            custom_commands: value.custom_commands,
            shell: value.shell,
        }
    }
}

impl AppPersistentConfig for AppConfig {
    type Raw = AppConfigRaw;

    fn get_type() -> ConfigType {
        ConfigType::App
    }
}
