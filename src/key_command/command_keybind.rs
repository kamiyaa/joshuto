use crate::config::AppKeyMapping;

use super::KeyCommand;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind(KeyCommand),
    CompositeKeybind(AppKeyMapping),
}

impl std::fmt::Display for CommandKeybind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandKeybind::SimpleKeybind(s) => write!(f, "{}", s),
            CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
        }
    }
}
