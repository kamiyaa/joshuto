use std::convert::From;
use std::io;

#[derive(Clone, Debug)]
pub enum AppErrorKind {
    // io related
    Io,

    // environment variable not found
    EnvVar,

    // parse error
    Parse,
    Clipboard,
    Config,

    Trash,

    Glob,

    Regex,

    InvalidParameters,
    StateError,

    UnrecognizedArgument,
    UnrecognizedCommand,

    UnknownError,

    InternalError,
}

impl From<io::ErrorKind> for AppErrorKind {
    fn from(_: io::ErrorKind) -> Self {
        Self::Io
    }
}

impl From<&globset::ErrorKind> for AppErrorKind {
    fn from(_: &globset::ErrorKind) -> Self {
        Self::Glob
    }
}

impl From<std::env::VarError> for AppErrorKind {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvVar
    }
}

impl From<toml::de::Error> for AppErrorKind {
    fn from(_: toml::de::Error) -> Self {
        Self::Config
    }
}
