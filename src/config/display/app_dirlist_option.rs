use crate::state::MatchState;

/// Display options valid pre AppDirectoryList in a JoshutoTab
#[derive(Clone, Debug)]
pub struct AppDirListDisplayOptions {
    pub filter_state: MatchState,
    pub depth: u8,
}

impl AppDirListDisplayOptions {
    /// Sets the active filter/search match state for this directory list.
    pub fn set_filter_state(&mut self, filter_state: MatchState) {
        self.filter_state = filter_state;
    }

    /// Returns the active filter/search match state for this directory list.
    pub fn filter_state_ref(&self) -> &MatchState {
        &self.filter_state
    }

    /// Sets how many levels deep this directory list is flattened (see `flat` command).
    pub fn set_depth(&mut self, depth: u8) {
        self.depth = depth;
    }

    /// Returns how many levels deep this directory list is flattened.
    pub fn depth(&self) -> u8 {
        self.depth
    }
}

impl std::default::Default for AppDirListDisplayOptions {
    fn default() -> Self {
        Self {
            filter_state: MatchState::None,
            depth: 0,
        }
    }
}
