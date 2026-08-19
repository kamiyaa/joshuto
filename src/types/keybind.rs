use std::collections::HashMap;

use ratatui::termion::event::Event;

use crate::types::command::Command;

/// Maps key events to the keybind (command or nested key sequence) they trigger.
pub type KeyMapping = HashMap<Event, CommandKeybind>;

/// What a key event does: run a fixed list of commands, or continue into a multi-key sequence.
#[derive(Debug)]
pub enum CommandKeybind {
    /// The key event directly runs `commands`.
    SimpleKeybind {
        commands: Vec<Command>,
        description: Option<String>,
    },
    /// The key event is the first of a multi-key sequence; look up the next key here.
    CompositeKeybind(KeyMapping),
}

impl std::fmt::Display for CommandKeybind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandKeybind::SimpleKeybind {
                commands: _,
                description: Some(desc),
            } => write!(f, "{}", desc),
            CommandKeybind::SimpleKeybind {
                commands,
                description: None,
            } => {
                for cmd in commands {
                    write!(f, "{}, ", cmd)?;
                }
                Ok(())
            }
            CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
        }
    }
}
