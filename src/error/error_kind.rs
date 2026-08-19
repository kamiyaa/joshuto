use std::convert::From;
use std::io;

/// Category of an [`AppError`](super::AppError), used to distinguish error causes without
/// carrying a full error object.
#[derive(Clone, Debug)]
pub enum AppErrorKind {
    /// An underlying `std::io` operation failed.
    Io,

    /// A required environment variable was not found.
    EnvVar,

    /// A value failed to parse.
    Parse,
    /// The system clipboard could not be read or written.
    Clipboard,
    /// A config file failed to parse or load.
    Config,

    /// A trash/recycle-bin operation failed.
    Trash,

    /// A glob pattern was invalid or failed to match.
    Glob,

    /// A regex pattern was invalid or failed to match.
    Regex,

    /// A command or option was given invalid parameters.
    InvalidParameters,
    /// joshuto's internal state was inconsistent or unexpected.
    StateError,

    /// A command argument was not recognized.
    UnrecognizedArgument,
    /// A command name was not recognized.
    UnrecognizedCommand,

    /// An error occurred that doesn't fit any other category.
    UnknownError,

    /// An internal invariant was violated; indicates a bug in joshuto itself.
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
