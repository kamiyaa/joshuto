use std::path::Path;
use std::{io, process::Command};

use crate::error::{AppError, AppErrorKind, AppResult};

pub struct Mimetype {
    _type: String,
    _subtype: String,
}

impl Mimetype {
    pub fn new(ttype: String, subtype: String) -> Self {
        Self {
            _type: ttype,
            _subtype: subtype,
        }
    }

    pub fn get_type(&self) -> &str {
        &self._type
    }

    pub fn get_subtype(&self) -> &str {
        &self._subtype
    }
}

pub fn get_mimetype(p: &Path) -> AppResult<Mimetype> {
    let res = Command::new("file")
        .arg("--mime-type")
        .arg("-Lb")
        .arg(p)
        .output();

    let output = res?;
    if !output.status.success() {
        let stderr_msg = String::from_utf8_lossy(&output.stderr).to_string();

        let error = AppError::new(AppErrorKind::Io(io::ErrorKind::InvalidInput), stderr_msg);
        return Err(error);
    }

    let stdout_msg = String::from_utf8_lossy(&output.stdout).to_string();
    match stdout_msg.trim().split_once('/') {
        Some((ttype, subtype)) => Ok(Mimetype::new(ttype.to_string(), subtype.to_string())),
        None => {
            let error = AppError::new(
                AppErrorKind::Io(io::ErrorKind::InvalidInput),
                "Unknown mimetype".to_string(),
            );
            Err(error)
        }
    }
}
