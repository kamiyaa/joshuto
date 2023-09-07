mod error_kind;
mod error_type;

pub use self::error_kind::AppErrorKind;
pub use self::error_type::AppError;

pub type AppResult<T = ()> = Result<T, AppError>;
