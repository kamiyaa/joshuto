extern crate fs_extra;
extern crate ncurses;

use std;

use std::fmt;

use joshuto;
use joshuto::command;

#[derive(Clone, Debug)]
pub struct ReloadDirList;

impl ReloadDirList {
    pub fn new() -> Self { ReloadDirList }
    pub fn command() -> &'static str { "reload_dir_list" }

    pub fn reload(context: &mut joshuto::JoshutoContext)
    {
        let curr_tab = &mut context.tabs[context.tab_index];
        curr_tab.reload_contents(&context.config_t.sort_type);
        curr_tab.refresh(&context.views, &context.theme_t, &context.config_t,
            &context.username, &context.hostname);
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
        ncurses::doupdate();
    }
}
