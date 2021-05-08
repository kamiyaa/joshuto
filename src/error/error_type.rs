use std::io;

use super::JoshutoErrorKind;

pub struct JoshutoError {
    _kind: JoshutoErrorKind,
    _cause: String,
}

#[allow(dead_code)]
impl JoshutoError {
    pub fn new(_kind: JoshutoErrorKind, _cause: String) -> Self {
        Self { _kind, _cause }
    }

    pub fn kind(&self) -> JoshutoErrorKind {
        self._kind
    }
}

impl std::fmt::Display for JoshutoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self._cause)
    }
}

impl std::convert::From<io::Error> for JoshutoError {
    fn from(err: io::Error) -> Self {
        Self {
            _kind: JoshutoErrorKind::from(err.kind()),
            _cause: err.to_string(),
        }
    }
}

impl std::convert::From<globset::Error> for JoshutoError {
    fn from(err: globset::Error) -> Self {
        Self {
            _kind: JoshutoErrorKind::from(err.kind()),
            _cause: err.to_string(),
        }
    }
}

impl std::convert::From<std::env::VarError> for JoshutoError {
    fn from(err: std::env::VarError) -> Self {
        Self {
            _kind: JoshutoErrorKind::from(err),
            _cause: "Environment variable not found".to_string(),
        }
    }
}

impl std::convert::From<trash::Error> for JoshutoError {
    fn from(err: trash::Error) -> Self {
        let err = match err {
            trash::Error::Unknown => {
                std::io::Error::new(std::io::ErrorKind::Other, "Unknown Error")
            }
            trash::Error::TargetedRoot => {
                std::io::Error::new(std::io::ErrorKind::Other, "Targeted Root")
            }
            trash::Error::CanonicalizePath { code: _ } => {
                std::io::Error::new(std::io::ErrorKind::NotFound, "Not found")
            }
            trash::Error::Remove { code: Some(1) } => std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Cannot move files to trash from mounted system",
            ),
            _ => std::io::Error::new(std::io::ErrorKind::Other, "Unknown Error"),
        };
        Self {
            _kind: JoshutoErrorKind::from(err.kind()),
            _cause: err.to_string(),
        }
    }
}
