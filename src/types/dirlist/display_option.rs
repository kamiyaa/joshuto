use crate::app_state::MatchContext;

/// Display options valid pre JoshutoDirList in a JoshutoTab
#[derive(Clone, Debug, Default)]
pub struct DirListDisplayOptions {
    filter_app_state: MatchContext,
    depth: u8,
}

impl DirListDisplayOptions {
    pub fn set_filter_app_state(&mut self, filter_app_state: MatchContext) {
        self.filter_app_state = filter_app_state;
    }

    pub fn filter_app_state_ref(&self) -> &MatchContext {
        &self.filter_app_state
    }

    pub fn set_depth(&mut self, depth: u8) {
        self.depth = depth;
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }
}