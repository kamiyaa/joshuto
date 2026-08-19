use std::path::Path;
use std::process::Command;

use crate::error::{AppError, AppErrorKind, AppResult};

/// A detected MIME type, split into its type and subtype (e.g. `"text"` / `"plain"`).
pub struct Mimetype {
    _type: String,
    _subtype: String,
}

impl Mimetype {
    /// Builds a `Mimetype` from its type and subtype strings.
    pub fn new(ttype: String, subtype: String) -> Self {
        Self {
            _type: ttype,
            _subtype: subtype,
        }
    }

    /// Returns the MIME type (e.g. `"text"`).
    pub fn get_type(&self) -> &str {
        &self._type
    }

    /// Returns the MIME subtype (e.g. `"plain"`).
    pub fn get_subtype(&self) -> &str {
        &self._subtype
    }
}

/// Detects the MIME type of file `p` by shelling out to `file --mime-type`.
pub fn get_mimetype(p: &Path) -> AppResult<Mimetype> {
    let res = Command::new("file")
        .arg("--mime-type")
        .arg("-Lb")
        .arg(p)
        .output();

    let output = res?;
    if !output.status.success() {
        let stderr_msg = String::from_utf8_lossy(&output.stderr).to_string();

        let error = AppError::new(AppErrorKind::Io, stderr_msg);
        return Err(error);
    }

    let stdout_msg = String::from_utf8_lossy(&output.stdout).to_string();
    match stdout_msg.trim().split_once('/') {
        Some((ttype, subtype)) => Ok(Mimetype::new(ttype.to_string(), subtype.to_string())),
        None => {
            let error = AppError::new(AppErrorKind::Io, "Unknown mimetype".to_string());
            Err(error)
        }
    }
}
