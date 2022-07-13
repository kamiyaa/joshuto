use crate::config::KeyMapping;
use crate::fs::FileType;
use crate::key_command::Command;
use std::collections::HashMap;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind(HashMap<Option<FileType>, Command>),
    CompositeKeybind(KeyMapping),
}

// impl std::fmt::Display for CommandKeybind {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             CommandKeybind::SimpleKeybind(s) => write!(f, "{}", s),
//             CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
//         }
//     }
// }
