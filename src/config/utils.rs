//! `serde(default = ...)` helper functions and custom deserializers shared across config types.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::path::{self, Path, PathBuf};

use crate::config::{LineMode, LineModeArgs};
use crate::error::{AppError, AppErrorCode, AppResult};
use crate::global;

/// Default `max_preview_size`: 2 MB.
pub const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}

/// Default `mode`: `"default"`.
pub fn default_mode() -> String {
    "default".to_string()
}

/// Returns `true`, for fields defaulting to enabled.
pub const fn default_true() -> bool {
    true
}

/// Default `scroll_offset`: 4.
pub const fn default_scroll_offset() -> usize {
    4
}

/// Default `shell`: `/bin/sh`.
pub fn default_shell() -> path::PathBuf {
    path::PathBuf::from("/bin/sh")
}

/// Deserializes a `|`-separated linemode string (e.g. `"size | mtime"`) into a [`LineMode`],
/// ignoring (and warning about) unrecognized entries instead of failing.
pub fn deserialize_line_mode<'de, D>(deserializer: D) -> Result<LineMode, D::Error>
where
    D: Deserializer<'de>,
{
    let line_mode_string: String = Deserialize::deserialize(deserializer)?;

    let mut line_mode = LineMode::empty();

    for mode in line_mode_string.split('|').map(|mode| mode.trim()) {
        match mode {
            "size" => line_mode.add_mode(LineModeArgs::Size),
            "mtime" => line_mode.add_mode(LineModeArgs::ModifyTime),
            "atime" => line_mode.add_mode(LineModeArgs::AccessTime),
            "user" => line_mode.add_mode(LineModeArgs::User),
            "group" => line_mode.add_mode(LineModeArgs::Group),
            "perm" => line_mode.add_mode(LineModeArgs::Permission),
            e => eprintln!("{e} is an unsupportted line mode, will be ignored"),
        }
    }

    Ok(line_mode)
}

/// Searches `directories` in order for a file named `file_name`, returning the first match.
pub fn search_directories<P>(directories: &[P], file_name: &str) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    directories
        .iter()
        .map(|path| path.as_ref().join(file_name))
        .find(|path| path.exists())
}

fn parse_file_to_config<T, S>(file_path: &Path) -> AppResult<S>
where
    T: DeserializeOwned + Into<S>,
{
    let file_contents = std::fs::read(file_path).map_err(|err| {
        let error_msg = "Failed to read config file";
        tracing::error!(?err, "{error_msg}");
        AppError::new(AppErrorCode::ConfigRead, error_msg.to_string())
    })?;
    let config = toml::from_slice::<T>(&file_contents).map_err(|err| {
        let error_msg = "Failed to parse config file";
        tracing::error!(?err, "{error_msg}");
        AppError::new(AppErrorCode::ConfigParse, error_msg.to_string())
    })?;
    Ok(config.into())
}

/// Finds, reads, and parses `file_name` from the config directory hierarchy, returning
/// `S::default()` (and printing a warning) if it's missing or fails to parse.
pub fn parse_config_or_default<T, S>(file_name: &str) -> S
where
    T: DeserializeOwned + Into<S>,
    S: std::default::Default,
{
    match search_directories(&global::CONFIG_HIERARCHY, file_name) {
        Some(file_path) => match parse_file_to_config::<T, S>(&file_path) {
            Ok(s) => s,
            Err(err) => {
                let error_msg = "Failed to read config";
                tracing::error!(?err, "{error_msg}");
                S::default()
            }
        },
        None => S::default(),
    }
}
