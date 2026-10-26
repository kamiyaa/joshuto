use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppErrorCode, AppResult};

/// How line numbers are displayed next to file entries: not at all, relative to the cursor, or
/// as absolute row numbers.
#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub enum LineNumberStyle {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}

/// A single metadata column that can be shown per file entry in a [`LineMode`](super::LineMode).
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineModeArgs {
    Size,
    ModifyTime,
    AccessTime,
    User,
    Group,
    Permission,
    #[default]
    Null,
}

impl AsRef<str> for LineModeArgs {
    fn as_ref(&self) -> &str {
        match self {
            LineModeArgs::Size => "size",
            LineModeArgs::ModifyTime => "mtime",
            LineModeArgs::AccessTime => "atime",
            LineModeArgs::User => "user",
            LineModeArgs::Group => "group",
            LineModeArgs::Permission => "perm",
            LineModeArgs::Null => unreachable!(),
        }
    }
}

/// An ordered, deduplicated set of up to 7 [`LineModeArgs`] columns to display per entry.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LineMode {
    pub mode: [LineModeArgs; 7],
    pub size: usize,
}

impl LineMode {
    /// Returns a `LineMode` showing every available column.
    pub const fn all() -> Self {
        Self {
            mode: [
                LineModeArgs::Size,
                LineModeArgs::ModifyTime,
                LineModeArgs::AccessTime,
                LineModeArgs::User,
                LineModeArgs::Group,
                LineModeArgs::Permission,
                LineModeArgs::Null,
            ],
            size: 7,
        }
    }

    /// Returns a `LineMode` showing no columns.
    pub const fn empty() -> Self {
        Self {
            mode: [LineModeArgs::Null; 7],
            size: 0,
        }
    }

    /// Appends `mode` if it isn't already present.
    pub fn add_mode(&mut self, mode: LineModeArgs) {
        if self.mode.contains(&mode) {
            return;
        }

        self.mode[self.size] = mode;
        self.size += 1;
    }
}

impl Default for LineMode {
    fn default() -> Self {
        let mut mode = [Default::default(); 7];
        mode[0] = LineModeArgs::Size;

        Self { size: 1, mode }
    }
}

impl LineMode {
    /// Parses a `|`-separated linemode spec (or `"all"`/`"none"`) from the `linemode` command
    /// or config file.
    pub fn from_string(name: &str) -> AppResult<LineMode> {
        if name == "all" {
            return Ok(LineMode::all());
        }
        if name == "none" {
            return Ok(LineMode::empty());
        }

        let mut line_mode = LineMode::empty();

        for mode in name.split('|').map(|mode| mode.trim()) {
            match mode {
                "size" => line_mode.add_mode(LineModeArgs::Size),
                "mtime" => line_mode.add_mode(LineModeArgs::ModifyTime),
                "atime" => line_mode.add_mode(LineModeArgs::AccessTime),
                "user" => line_mode.add_mode(LineModeArgs::User),
                "group" => line_mode.add_mode(LineModeArgs::Group),
                "perm" => line_mode.add_mode(LineModeArgs::Permission),
                value => {
                    let error_msg = "Unknown LineMode value";
                    tracing::error!(value, "{error_msg}");
                    return Err(AppError::new(AppErrorCode::Parse, error_msg.to_string()));
                }
            }
        }
        Ok(line_mode)
    }

    /// Formats the active columns as a `" | "`-joined string.
    pub fn as_string(&self) -> String {
        let modes: Vec<&str> = self
            .mode
            .iter()
            .take(self.size)
            .map(AsRef::as_ref)
            .collect();

        modes.join(" | ")
    }
}
