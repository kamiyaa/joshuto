use crate::types::state::MatchState;

/// Display options valid pre JoshutoDirList in a JoshutoTab
#[derive(Clone, Debug, Default)]
pub struct DirListDisplayOptions {
    filter_state: MatchState,
    depth: u8,
}

impl DirListDisplayOptions {
    pub fn set_filter_state(&mut self, filter_state: MatchState) {
        self.filter_state = filter_state;
    }

    pub fn filter_state_ref(&self) -> &MatchState {
        &self.filter_state
    }

    pub fn set_depth(&mut self, depth: u8) {
        self.depth = depth;
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }
}
