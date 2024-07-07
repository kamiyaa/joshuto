use rustyline::history::{History, MemHistory};

pub struct CommandLineState {
    pub history: MemHistory,
}

impl std::default::Default for CommandLineState {
    fn default() -> Self {
        Self {
            history: MemHistory::new(),
        }
    }
}

impl CommandLineState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn history_ref(&self) -> &dyn History {
        &self.history
    }
    pub fn history_mut(&mut self) -> &mut dyn History {
        &mut self.history
    }
}
