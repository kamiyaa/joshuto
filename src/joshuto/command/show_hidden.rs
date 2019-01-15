extern crate fs_extra;
extern crate ncurses;

use std;
use std::fmt;

use joshuto;
use joshuto::command;

#[derive(Clone, Debug)]
pub struct ToggleHiddenFiles;

impl ToggleHiddenFiles {
    pub fn new() -> Self { ToggleHiddenFiles }
    pub fn command() -> &'static str { "toggle_hidden" }
    pub fn toggle_hidden(context: &mut joshuto::JoshutoContext)
    {
        let opposite = !context.config_t.sort_type.show_hidden();
        context.config_t.sort_type.set_show_hidden(opposite);

        for tab in &mut context.tabs {
            tab.history.depecrate_all_entries();
            tab.reload_contents(&context.config_t.sort_type);
        }
    }
}

impl command::JoshutoCommand for ToggleHiddenFiles {}

impl std::fmt::Display for ToggleHiddenFiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl command::Runnable for ToggleHiddenFiles {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        Self::toggle_hidden(context);
        let curr_tab = &mut context.tabs[context.tab_index];
        curr_tab.reload_contents(&context.config_t.sort_type);
        curr_tab.refresh(&context.views, &context.theme_t, &context.config_t,
            &context.username, &context.hostname);

        ncurses::doupdate();
    }
}
