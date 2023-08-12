use crate::config::KeyMapping;
use crate::key_command::Command;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind {
        command: Command,
        description: Option<String>,
    },
    CompositeKeybind(KeyMapping),
}

impl std::fmt::Display for CommandKeybind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandKeybind::SimpleKeybind {
                command: _,
                description: Some(desc),
            } => write!(f, "{}", desc),
            CommandKeybind::SimpleKeybind {
                command: cmd,
                description: None,
            } => write!(f, "{}", cmd),
            CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
        }
    }
}
