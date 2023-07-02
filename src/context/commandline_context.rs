use rustyline::history::{History, MemHistory};

pub struct CommandLineContext {
    history: MemHistory,
}

impl std::default::Default for CommandLineContext {
    fn default() -> Self {
        Self {
            history: MemHistory::new(),
        }
    }
}

impl CommandLineContext {
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
