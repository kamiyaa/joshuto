use std::convert::From;
use std::io;

#[derive(Clone, Debug)]
pub enum AppErrorKind {
    // io related
    Io(io::ErrorKind),

    // environment variable not found
    EnvVarNotPresent,

    // parse error
    ParseError,
    ClipboardError,
    TomlDeError(toml::de::Error),

    TrashError,

    Glob,

    Regex,

    InvalidParameters,

    UnrecognizedArgument,
    UnrecognizedCommand,

    UnknownError,
}

impl From<io::ErrorKind> for AppErrorKind {
    fn from(err: io::ErrorKind) -> Self {
        Self::Io(err)
    }
}

impl From<&globset::ErrorKind> for AppErrorKind {
    fn from(_: &globset::ErrorKind) -> Self {
        Self::Glob
    }
}

impl From<std::env::VarError> for AppErrorKind {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvVarNotPresent
    }
}

impl From<toml::de::Error> for AppErrorKind {
    fn from(err: toml::de::Error) -> Self {
        Self::TomlDeError(err)
    }
}
