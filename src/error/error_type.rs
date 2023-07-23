use std::convert::From;
use std::io;

use super::JoshutoErrorKind;

#[derive(Clone, Debug)]
pub struct JoshutoError {
    _kind: JoshutoErrorKind,
    _cause: String,
}

#[allow(dead_code)]
impl JoshutoError {
    pub fn new(_kind: JoshutoErrorKind, _cause: String) -> Self {
        Self { _kind, _cause }
    }

    pub fn kind(&self) -> &JoshutoErrorKind {
        &self._kind
    }
}

impl std::fmt::Display for JoshutoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self._cause)
    }
}

impl From<io::Error> for JoshutoError {
    fn from(err: io::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: JoshutoErrorKind::from(err.kind()),
            _cause: cause,
        }
    }
}

impl From<globset::Error> for JoshutoError {
    fn from(err: globset::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: JoshutoErrorKind::from(err.kind()),
            _cause: cause,
        }
    }
}

impl From<std::env::VarError> for JoshutoError {
    fn from(err: std::env::VarError) -> Self {
        let cause = err.to_string();
        Self {
            _kind: JoshutoErrorKind::from(err),
            _cause: cause,
        }
    }
}

#[cfg(feature = "recycle_bin")]
impl From<trash::Error> for JoshutoError {
    fn from(err: trash::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: JoshutoErrorKind::TrashError,
            _cause: cause,
        }
    }
}

impl From<toml::de::Error> for JoshutoError {
    fn from(err: toml::de::Error) -> Self {
        let cause = err.to_string();
        Self {
            _kind: JoshutoErrorKind::from(err),
            _cause: cause,
        }
    }
}
