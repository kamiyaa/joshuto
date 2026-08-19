use std::convert::From;
use std::io;

use super::AppErrorKind;

/// joshuto's catch-all error type: an [`AppErrorKind`] category plus a human-readable cause.
#[derive(Clone, Debug)]
pub struct AppError {
    _kind: AppErrorKind,
    _cause: String,
}

impl AppError {
    /// Builds an `AppError` from an explicit kind and cause message.
    pub fn new(_kind: AppErrorKind, _cause: String) -> Self {
        Self { _kind, _cause }
    }

    /// Builds an `AppError` with kind [`AppErrorKind::UnknownError`] from any displayable cause.
    pub fn error(cause: impl ToString) -> Self {
        Self::new(AppErrorKind::UnknownError, cause.to_string())
    }

    /// Shorthand for returning `Err(AppError::error(cause))` from a fallible function.
    pub fn fail<T>(cause: impl ToString) -> Result<T, AppError> {
        Err(Self::error(cause))
    }

    /// Returns this error's category.
    #[allow(dead_code)]
    pub fn kind(&self) -> &AppErrorKind {
        &self._kind
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self._cause)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: AppErrorKind::from(err.kind()),
            _cause: cause,
        }
    }
}

impl From<globset::Error> for AppError {
    fn from(err: globset::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: AppErrorKind::from(err.kind()),
            _cause: cause,
        }
    }
}

impl From<regex::Error> for AppError {
    fn from(err: regex::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: AppErrorKind::Regex,
            _cause: cause,
        }
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        let cause = err.to_string();
        Self {
            _kind: AppErrorKind::from(err),
            _cause: cause,
        }
    }
}

impl From<toml::de::Error> for AppError {
    fn from(err: toml::de::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: AppErrorKind::from(err),
            _cause: cause,
        }
    }
}
