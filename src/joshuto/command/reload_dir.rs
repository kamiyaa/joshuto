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

    pub fn reload(context: &mut joshuto::JoshutoContext)
    {
        context.reload_dirlists();
        ui::refresh(&context);
        ncurses::doupdate();
    }
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
        Self::reload(context);
    }
}
