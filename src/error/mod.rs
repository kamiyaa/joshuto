use std::io;

/// Every kind of error joshuto can report.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppErrorCode {
    /// An underlying `std::io` operation failed.
    Io,
    /// A required environment variable was not found or was invalid.
    EnvVar,
    /// Changing the process's working directory failed.
    SetCurrentDirectory,

    /// A config file could not be read.
    ConfigRead,
    /// A config file failed to parse.
    ConfigParse,
    /// A built-in default config failed to parse; indicates a bug in joshuto itself.
    DefaultConfigParse,
    /// No built-in default exists for the requested config type.
    NoDefaultConfig,
    /// Two keybindings conflict with each other.
    AmbiguousKeybinding,

    /// A value failed to parse.
    Parse,
    /// A glob pattern was invalid.
    Glob,
    /// A regex pattern was invalid.
    Regex,

    /// The system clipboard could not be read or written.
    Clipboard,
    /// A trash/recycle-bin operation failed.
    Trash,

    /// A command or option was given invalid parameters.
    InvalidParameters,
    /// A command argument was not recognized.
    UnrecognizedArgument,
    /// A command name was not recognized.
    UnrecognizedCommand,
    /// joshuto's internal state was inconsistent or unexpected.
    StateError,

    /// An internal invariant was violated; indicates a bug in joshuto itself.
    Internal,
    /// An error occurred that doesn't fit any other category.
    Unknown,
}

/// joshuto's catch-all error type: an [`AppErrorCode`] plus a human-readable message.
#[derive(Clone, Debug)]
pub struct AppError {
    pub code: AppErrorCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: AppErrorCode, message: String) -> Self {
        Self { code, message }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        Self::new(AppErrorCode::Io, err.to_string())
    }
}

impl From<globset::Error> for AppError {
    fn from(err: globset::Error) -> Self {
        Self::new(AppErrorCode::Glob, err.to_string())
    }
}

impl From<regex::Error> for AppError {
    fn from(err: regex::Error) -> Self {
        Self::new(AppErrorCode::Regex, err.to_string())
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        Self::new(AppErrorCode::EnvVar, err.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(err: toml::de::Error) -> Self {
        Self::new(AppErrorCode::ConfigParse, err.to_string())
    }
}

pub type AppResult<T = ()> = Result<T, AppError>;
