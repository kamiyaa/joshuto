use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::ui::TuiBackend;

#[derive(Clone, Debug)]
pub struct ToggleHiddenFiles;

impl ToggleHiddenFiles {
    pub fn new() -> Self {
        ToggleHiddenFiles
    }
    pub const fn command() -> &'static str {
        "toggle_hidden"
    }
    pub fn toggle_hidden(context: &mut JoshutoContext) {
        let opposite = !context.config_t.sort_option.show_hidden;
        context.config_t.sort_option.show_hidden = opposite;

        for tab in &mut context.tabs {
            tab.history.depreciate_all_entries();
            match tab.curr_list_mut() {
                Some(s) => s.depreciate(),
                None => {}
            }
        }
    }
}

impl JoshutoCommand for ToggleHiddenFiles {}

impl std::fmt::Display for ToggleHiddenFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for ToggleHiddenFiles {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        Self::toggle_hidden(context);
        ReloadDirList::new().execute(context, backend)
    }
}
