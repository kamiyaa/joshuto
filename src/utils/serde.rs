//! `serde(default = ...)` helper functions and custom deserializers shared across config types.

use serde::{Deserialize, Deserializer};
use std::path;

use crate::types::option::line_mode::{LineMode, LineModeArgs};

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
