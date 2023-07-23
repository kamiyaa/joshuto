use std::convert::From;
use std::io;

#[derive(Clone, Debug)]
pub enum JoshutoErrorKind {
    // io related
    Io(io::ErrorKind),

    // environment variable not found
    EnvVarNotPresent,

    // parse error
    ParseError,
    ClipboardError,
    TomlDeError(toml::de::Error),

    #[cfg(feature = "recycle_bin")]
    TrashError,

    Glob,

    InvalidParameters,

    UnrecognizedArgument,
    UnrecognizedCommand,
}

impl From<io::ErrorKind> for JoshutoErrorKind {
    fn from(err: io::ErrorKind) -> Self {
        Self::Io(err)
    }
}

impl From<&globset::ErrorKind> for JoshutoErrorKind {
    fn from(_: &globset::ErrorKind) -> Self {
        Self::Glob
    }
}

impl From<std::env::VarError> for JoshutoErrorKind {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvVarNotPresent
    }
}

impl From<toml::de::Error> for JoshutoErrorKind {
    fn from(err: toml::de::Error) -> Self {
        Self::TomlDeError(err)
    }
}
