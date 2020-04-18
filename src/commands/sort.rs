use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::ui::TuiBackend;

use crate::util::load_child::LoadChild;
use crate::util::sort::SortType;

#[derive(Clone, Debug)]
pub struct Sort {
    sort_method: SortType,
}

impl Sort {
    pub fn new(sort_method: SortType) -> Self {
        Self { sort_method }
    }
    pub const fn command() -> &'static str {
        "sort"
    }
}

impl JoshutoCommand for Sort {}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.sort_method.as_str())
    }
}

impl JoshutoRunnable for Sort {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        context.config_t.sort_option.sort_method = self.sort_method;
        for tab in context.tabs.iter_mut() {
            tab.history.depreciate_all_entries();
        }
        ReloadDirList::soft_reload(context.curr_tab_index, context)?;
        LoadChild::load_child(context)?;
        Ok(())
    }
}
