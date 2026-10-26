use std::collections::HashMap;

use ratatui::termion::event::Event;

use crate::app_command::AppCommand;

/// Maps key events to the keybind (command or nested key sequence) they trigger.
pub type KeyMapping = HashMap<Event, KeyEventAction>;

/// What a key event does: run a fixed list of commands, or continue into a multi-key sequence.
#[derive(Debug)]
pub enum KeyEventAction {
    /// The key event directly runs `commands`.
    Base {
        commands: Vec<AppCommand>,
        description: Option<String>,
    },
    /// The key event is the first of a multi-key sequence; look up the next key here.
    Recursive(KeyMapping),
}

impl std::fmt::Display for KeyEventAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KeyEventAction::Base {
                commands: _,
                description: Some(desc),
            } => write!(f, "{}", desc),
            KeyEventAction::Base {
                commands,
                description: None,
            } => {
                for cmd in commands {
                    write!(f, "{}, ", cmd)?;
                }
                Ok(())
            }
            KeyEventAction::Recursive(_) => write!(f, "..."),
        }
    }
}
