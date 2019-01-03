extern crate fs_extra;
extern crate ncurses;

use std;
use std::fmt;
use std::path;
use std::process;

use joshuto;
use joshuto::command;
use joshuto::ui;

#[derive(Debug)]
pub struct ToggleHiddenFiles;

impl ToggleHiddenFiles {
    pub fn new() -> Self { ToggleHiddenFiles }
    pub fn command() -> &'static str { "toggle_hidden" }
}

impl command::JoshutoCommand for ToggleHiddenFiles {}

impl std::fmt::Display for ToggleHiddenFiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for ToggleHiddenFiles {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        {
            let opposite = !context.config_t.sort_type.show_hidden();
            context.config_t.sort_type.set_show_hidden(opposite);
            context.history.depecrate_all_entries();

            if let Some(s) = context.curr_list.as_mut() {
                s.update_needed = true;
            }

            if let Some(s) = context.preview_list.as_mut() {
                s.update_needed = true;
            }

            if let Some(s) = context.parent_list.as_mut() {
                s.update_needed = true;
            }
        }

        context.reload_dirlists();

        ui::redraw_view(&context.views.left_win, context.parent_list.as_ref());
        ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
        ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

        ui::redraw_status(&context.views, context.curr_list.as_ref(),
                &context.curr_path,
                &context.config_t.username, &context.config_t.hostname);

        ncurses::doupdate();
    }
}
