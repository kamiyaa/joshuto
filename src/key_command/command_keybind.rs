use crate::config::KeyMapping;
use crate::key_command::Command;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind(Command),
    CompositeKeybind(KeyMapping),
}

impl std::fmt::Display for CommandKeybind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandKeybind::SimpleKeybind(s) => write!(f, "{}", s),
            CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
        }
    }
}
