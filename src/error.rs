pub enum JoshutoError {
    IO(std::io::Error),
}

/*
pub enum KeymapErrorKind {
    Parse,
    UnknownArgument,
    UnknownCommand,
}
*/

pub struct KeymapError {
    pub command: Option<&'static str>,
    pub error: String,
}

impl KeymapError {
    pub fn new(command: Option<&'static str>, error: String) -> Self {
        KeymapError { command, error }
    }
}

impl std::fmt::Display for KeymapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.command {
            Some(s) => write!(f, "{}: {}", s, self.error),
            None => write!(f, "{}", self.error),
        }
    }
}
