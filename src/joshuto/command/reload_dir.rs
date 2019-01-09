extern crate fs_extra;
extern crate ncurses;

use std;

use std::fmt;

use joshuto;
use joshuto::ui;
use joshuto::command;

#[derive(Clone, Debug)]
pub struct ReloadDirList;

impl ReloadDirList {
    pub fn new() -> Self { ReloadDirList }
    pub fn command() -> &'static str { "reload_dir_list" }
}

impl command::JoshutoCommand for ReloadDirList {}

impl std::fmt::Display for ReloadDirList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl command::Runnable for ReloadDirList {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        context.reload_dirlists();

        let curr_tab = &mut context.tabs[context.tab_index];

        ui::redraw_view(&context.views.left_win, curr_tab.parent_list.as_ref());
        ui::redraw_view(&context.views.mid_win, curr_tab.curr_list.as_ref());
        ui::redraw_view(&context.views.right_win, curr_tab.preview_list.as_ref());

        ui::redraw_status(&context.views, curr_tab.curr_list.as_ref(),
                &curr_tab.curr_path,
                &context.username, &context.hostname);

        ncurses::doupdate();
    }
}
