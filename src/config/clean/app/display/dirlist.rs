use crate::context::MatchContext;

/// Display options valid pre JoshutoDirList in a JoshutoTab
#[derive(Clone, Debug, Default)]
pub struct DirListDisplayOptions {
    filter_context: MatchContext,
    depth: u8,
}

impl DirListDisplayOptions {
    pub fn set_filter_context(&mut self, filter_context: MatchContext) {
        self.filter_context = filter_context;
    }

    pub fn filter_context_ref(&self) -> &MatchContext {
        &self.filter_context
    }

    pub fn set_depth(&mut self, depth: u8) {
        self.depth = depth;
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }
}
