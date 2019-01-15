use std;
use std::fmt;
use std::path;
use std::env;

use joshuto;
use joshuto::command;
use joshuto::structs::JoshutoTab;

#[derive(Clone, Debug)]
pub struct NewTab;

impl NewTab {
    pub fn new() -> Self { NewTab }
    pub const fn command() -> &'static str { "new_tab" }

    pub fn new_tab(context: &mut joshuto::JoshutoContext)
    {
        let curr_path: path::PathBuf = match env::current_dir() {
            Ok(path) => { path },
            Err(e) => {
                eprintln!("{}", e);
                return;
            },
        };

        let tab = JoshutoTab::new(curr_path, &context.config_t.sort_type);

        context.tabs.push(tab);
        context.tab_index = context.tabs.len() - 1;

        command::TabSwitch::tab_switch(context.tabs.len() as i32 - 1, context);
    }
}

impl command::JoshutoCommand for NewTab {}

impl std::fmt::Display for NewTab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl command::Runnable for NewTab {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        Self::new_tab(context);
    }
}

#[derive(Clone, Debug)]
pub struct CloseTab;

impl CloseTab {
    pub fn new() -> Self { CloseTab }
    pub const fn command() -> &'static str { "close_tab" }

    pub fn close_tab(context: &mut joshuto::JoshutoContext)
    {
        if context.tabs.len() <= 1 {
            return;
        }

        context.tabs.remove(context.tab_index);
        if context.tab_index > 0 {
            context.tab_index = context.tab_index - 1;
        }
        command::TabSwitch::tab_switch(context.tab_index as i32, context);
    }
}

impl command::JoshutoCommand for CloseTab {}

impl std::fmt::Display for CloseTab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl command::Runnable for CloseTab {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        Self::close_tab(context);
    }
}
