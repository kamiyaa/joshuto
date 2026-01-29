use std::collections::HashMap;

use ratatui::termion::event::Event;

use crate::types::command::Command;

pub type KeyMapping = HashMap<Event, CommandKeybind>;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind {
        commands: Vec<Command>,
        description: Option<String>,
    },
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
