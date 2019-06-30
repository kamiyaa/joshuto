use std::io::ErrorKind;

#[derive(Copy, Clone, Debug)]
pub enum JoshutoErrorKind {
    IONotFound,
    IOPermissionDenied,
    IOConnectionRefused,
    IOConnectionReset,
    IOConnectionAborted,
    IONotConnected,
    IOAddrInUse,
    IOAddrNotAvailable,
    IOBrokenPipe,
    IOAlreadyExists,
    IOWouldBlock,
    IOInvalidInput,

    // also used for invalid arguments
    IOInvalidData,
    IOTimedOut,
    IOWriteZero,
    IOInterrupted,
    IOOther,
    IOUnexpectedEof,

    // environment variable not found
    EnvVarNotPresent,

    ParseError,
    UnknownCommand,
}

impl std::convert::From<ErrorKind> for JoshutoErrorKind {
    fn from(err: ErrorKind) -> Self {
        match err {
            ErrorKind::NotFound => JoshutoErrorKind::IONotFound,
            ErrorKind::PermissionDenied => JoshutoErrorKind::IOPermissionDenied,
            ErrorKind::ConnectionRefused => JoshutoErrorKind::IOConnectionRefused,
            ErrorKind::ConnectionReset => JoshutoErrorKind::IOConnectionReset,
            ErrorKind::ConnectionAborted => JoshutoErrorKind::IOConnectionAborted,
            ErrorKind::NotConnected => JoshutoErrorKind::IONotConnected,
            ErrorKind::AddrInUse => JoshutoErrorKind::IOAddrInUse,
            ErrorKind::AddrNotAvailable => JoshutoErrorKind::IOAddrNotAvailable,
            ErrorKind::BrokenPipe => JoshutoErrorKind::IOBrokenPipe,
            ErrorKind::AlreadyExists => JoshutoErrorKind::IOAlreadyExists,
            ErrorKind::WouldBlock => JoshutoErrorKind::IOWouldBlock,
            ErrorKind::InvalidInput => JoshutoErrorKind::IOInvalidInput,
            ErrorKind::InvalidData => JoshutoErrorKind::IOInvalidData,
            ErrorKind::TimedOut => JoshutoErrorKind::IOTimedOut,
            ErrorKind::WriteZero => JoshutoErrorKind::IOWriteZero,
            ErrorKind::Interrupted => JoshutoErrorKind::IOInterrupted,
            ErrorKind::UnexpectedEof => JoshutoErrorKind::IOUnexpectedEof,
            ErrorKind::Other => JoshutoErrorKind::IOOther,
            _ => JoshutoErrorKind::IOOther,
        }
    }
}

pub struct JoshutoError {
    _kind: JoshutoErrorKind,
    _cause: String,
}

impl JoshutoError {
    pub fn new(_kind: JoshutoErrorKind, _cause: String) -> Self {
        JoshutoError { _kind, _cause }
    }

    pub fn kind(&self) -> JoshutoErrorKind {
        self._kind
    }

    pub fn cause(&self) -> &str {
        self._cause.as_str()
    }
}

impl std::string::ToString for JoshutoError {
    fn to_string(&self) -> String {
        self._cause.clone()
    }
}

impl std::convert::From<std::io::Error> for JoshutoError {
    fn from(err: std::io::Error) -> Self {
        JoshutoError {
            _kind: JoshutoErrorKind::from(err.kind()),
            _cause: err.to_string(),
        }
    }
}

pub type JoshutoResult<T> = Result<T, JoshutoError>;
