use crate::config::KeyMapping;
use crate::fs::FileType;
use crate::key_command::Command;
use std::collections::HashMap;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind(HashMap<Option<FileType>, Command>),
    CompositeKeybind(KeyMapping),
}
