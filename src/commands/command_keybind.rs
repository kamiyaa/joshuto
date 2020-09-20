use crate::config::JoshutoCommandMapping;
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

use super::KeyCommand;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind(KeyCommand),
    CompositeKeybind(JoshutoCommandMapping),
}

impl std::fmt::Display for CommandKeybind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandKeybind::SimpleKeybind(s) => write!(f, "{}", s),
            CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
        }
    }
}

pub trait JoshutoRunnable {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()>;
}

pub trait JoshutoCommand: JoshutoRunnable + std::fmt::Display + std::fmt::Debug {}
