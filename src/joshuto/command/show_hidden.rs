extern crate fs_extra;
extern crate ncurses;

use std;
use std::fmt;

use joshuto;
use joshuto::command;
use joshuto::ui;

#[derive(Clone, Debug)]
pub struct ToggleHiddenFiles;

impl ToggleHiddenFiles {
    pub fn new() -> Self { ToggleHiddenFiles }
    pub fn command() -> &'static str { "toggle_hidden" }
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
        {
            let opposite = !context.config_t.sort_type.show_hidden();
            context.config_t.sort_type.set_show_hidden(opposite);

            for tab in &mut context.tabs {
                tab.history.depecrate_all_entries();
                if let Some(s) = tab.curr_list.as_mut() {
                    s.update(&context.config_t.sort_type);
                }

                if let Some(s) = tab.parent_list.as_mut() {
                    s.update(&context.config_t.sort_type);
                }
            }
        }

        ui::refresh(context);

        ncurses::doupdate();
    }
}
