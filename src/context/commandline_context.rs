use rustyline::history;

pub struct CommandLineContext {
    history: history::History,
}

impl std::default::Default for CommandLineContext {
    fn default() -> Self {
        Self {
            history: history::History::new(),
        }
    }
}

impl CommandLineContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn history_ref(&self) -> &history::History {
        &self.history
    }
    pub fn history_mut(&mut self) -> &mut history::History {
        &mut self.history
    }
}
