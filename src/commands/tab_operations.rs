use std::env;
use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, Quit, TabSwitch};
use crate::context::JoshutoContext;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub struct NewTab;

impl NewTab {
    pub fn new() -> Self {
        NewTab
    }
    pub const fn command() -> &'static str {
        "new_tab"
    }

    pub fn new_tab(context: &mut JoshutoContext, view: &JoshutoView) {
        let curr_path: path::PathBuf = match env::current_dir() {
            Ok(path) => path,
            Err(e) => {
                ui::wprint_err(&view.bot_win, e.to_string().as_str());
                return;
            }
        };

        match JoshutoTab::new(curr_path, &context.config_t.sort_type) {
            Ok(tab) => {
                context.tabs.push(tab);
                context.curr_tab_index = context.tabs.len() - 1;

                TabSwitch::tab_switch(context.tabs.len() as i32 - 1, context, view);
            }
            Err(e) => {
                ui::wprint_err(&view.bot_win, e.to_string().as_str());
            }
        };
    }
}

impl JoshutoCommand for NewTab {}

impl std::fmt::Display for NewTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for NewTab {
    fn execute(&self, context: &mut JoshutoContext, view: &JoshutoView) {
        Self::new_tab(context, view);
        ncurses::doupdate();
    }
}

#[derive(Clone, Debug)]
pub struct CloseTab;

impl CloseTab {
    pub fn new() -> Self {
        CloseTab
    }
    pub const fn command() -> &'static str {
        "close_tab"
    }

    pub fn close_tab(context: &mut JoshutoContext, view: &JoshutoView) {
        if context.tabs.len() <= 1 {
            Quit::quit(context, view);
            return;
        }

        let _ = context.tabs.remove(context.curr_tab_index);
        if context.curr_tab_index > 0 {
            context.curr_tab_index -= 1;
        }
        TabSwitch::tab_switch(context.curr_tab_index as i32, context, view);
    }
}

impl JoshutoCommand for CloseTab {}

impl std::fmt::Display for CloseTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CloseTab {
    fn execute(&self, context: &mut JoshutoContext, view: &JoshutoView) {
        Self::close_tab(context, view);
        ncurses::doupdate();
    }
}
