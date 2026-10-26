use rustyline::history::{History, MemHistory};

/// State for the `:` command line, currently just its input history.
pub struct CommandLineState {
    pub history: MemHistory,
}

impl std::default::Default for CommandLineState {
    fn default() -> Self {
        let mut history = MemHistory::new();
        let _ = history.set_max_len(20);
        Self { history }
    }
}

impl CommandLineState {
    /// Creates a command-line state with an empty, capped history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the command-line input history.
    pub fn history_ref(&self) -> &dyn History {
        &self.history
    }
    /// Returns a mutable reference to the command-line input history.
    pub fn history_mut(&mut self) -> &mut dyn History {
        &mut self.history
    }
}
