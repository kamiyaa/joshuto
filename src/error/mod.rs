mod error;
mod error_kind;

pub use self::error::JoshutoError;
pub use self::error_kind::JoshutoErrorKind;

pub type JoshutoResult<T> = Result<T, JoshutoError>;
