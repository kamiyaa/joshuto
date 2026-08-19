mod error_kind;
mod error_type;

pub use self::error_kind::AppErrorKind;
pub use self::error_type::AppError;

/// Convenience alias for `Result<T, AppError>`, used throughout joshuto for fallible operations.
pub type AppResult<T = ()> = Result<T, AppError>;
