extern crate fs_extra;
extern crate ncurses;

use std;

use std::fmt;

use joshuto;
use joshuto::ui;
use joshuto::command;

#[derive(Debug)]
pub struct ReloadDirList;

impl ReloadDirList {
    pub fn new() -> Self { ReloadDirList }
    pub fn command() -> &'static str { "reload_dir_list" }
}

impl command::JoshutoCommand for ReloadDirList {}

impl std::fmt::Display for ReloadDirList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for ReloadDirList {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
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
