use std::io::ErrorKind;

#[derive(Copy, Clone, Debug)]
pub enum JoshutoErrorKind {
    IoNotFound,
    IoPermissionDenied,
    IoConnectionRefused,
    IoConnectionReset,
    IoConnectionAborted,
    IoNotConnected,
    IoAddrInUse,
    IoAddrNotAvailable,
    IoBrokenPipe,
    IoAlreadyExists,
    IoWouldBlock,
    IoInvalidInput,

    // also used for invalid arguments
    IoInvalidData,
    IoTimedOut,
    IoWriteZero,
    IoInterrupted,
    IoOther,
    IoUnexpectedEof,

    // environment variable not found
    EnvVarNotPresent,

    ParseError,
    ClipboardError,

    UnknownCommand,
}

impl std::convert::From<ErrorKind> for JoshutoErrorKind {
    fn from(err: ErrorKind) -> Self {
        match err {
            ErrorKind::NotFound => JoshutoErrorKind::IoNotFound,
            ErrorKind::PermissionDenied => JoshutoErrorKind::IoPermissionDenied,
            ErrorKind::ConnectionRefused => JoshutoErrorKind::IoConnectionRefused,
            ErrorKind::ConnectionReset => JoshutoErrorKind::IoConnectionReset,
            ErrorKind::ConnectionAborted => JoshutoErrorKind::IoConnectionAborted,
            ErrorKind::NotConnected => JoshutoErrorKind::IoNotConnected,
            ErrorKind::AddrInUse => JoshutoErrorKind::IoAddrInUse,
            ErrorKind::AddrNotAvailable => JoshutoErrorKind::IoAddrNotAvailable,
            ErrorKind::BrokenPipe => JoshutoErrorKind::IoBrokenPipe,
            ErrorKind::AlreadyExists => JoshutoErrorKind::IoAlreadyExists,
            ErrorKind::WouldBlock => JoshutoErrorKind::IoWouldBlock,
            ErrorKind::InvalidInput => JoshutoErrorKind::IoInvalidInput,
            ErrorKind::InvalidData => JoshutoErrorKind::IoInvalidData,
            ErrorKind::TimedOut => JoshutoErrorKind::IoTimedOut,
            ErrorKind::WriteZero => JoshutoErrorKind::IoWriteZero,
            ErrorKind::Interrupted => JoshutoErrorKind::IoInterrupted,
            ErrorKind::UnexpectedEof => JoshutoErrorKind::IoUnexpectedEof,
            ErrorKind::Other => JoshutoErrorKind::IoOther,
            _ => JoshutoErrorKind::IoOther,
        }
    }
}

pub struct JoshutoError {
    _kind: JoshutoErrorKind,
    _cause: String,
}

#[allow(dead_code)]
impl JoshutoError {
    pub fn new(_kind: JoshutoErrorKind, _cause: String) -> Self {
        JoshutoError { _kind, _cause }
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

impl std::convert::From<std::io::Error> for JoshutoError {
    fn from(err: std::io::Error) -> Self {
        JoshutoError {
            _kind: JoshutoErrorKind::from(err.kind()),
            _cause: err.to_string(),
        }
    }
}

pub type JoshutoResult<T> = Result<T, JoshutoError>;
