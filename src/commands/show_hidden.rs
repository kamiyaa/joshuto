use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::preview;
use crate::window::JoshutoView;

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
            tab.history.depecrate_all_entries();
            tab.reload_contents(&context.config_t.sort_option);
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
    fn execute(&self, context: &mut JoshutoContext, view: &JoshutoView) {
        Self::toggle_hidden(context);
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab.reload_contents(&context.config_t.sort_option);
        curr_tab.refresh(
            view,
            &context.config_t,
            &context.username,
            &context.hostname,
        );
        preview::preview_file(
            &mut context.tabs[context.curr_tab_index],
            view,
            &context.config_t,
        );
        ncurses::doupdate();
    }
}
