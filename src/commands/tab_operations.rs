use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, Quit, TabSwitch};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::tab::JoshutoTab;
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

    pub fn new_tab(context: &mut JoshutoContext, view: &JoshutoView) -> Result<(), JoshutoError> {
        let curr_path: path::PathBuf = match dirs::home_dir() {
            Some(path) => path,
            None => {
                let err =
                    std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find home directory");
                return Err(JoshutoError::IO(err));
            }
        };

        match JoshutoTab::new(curr_path, &context.config_t.sort_option) {
            Ok(tab) => {
                context.tabs.push(tab);
                context.curr_tab_index = context.tabs.len() - 1;

                match TabSwitch::tab_switch(context.curr_tab_index, context, view) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(JoshutoError::IO(e)),
                }
            }
            Err(e) => Err(JoshutoError::IO(e)),
        }
    }
}

impl JoshutoCommand for NewTab {}

impl std::fmt::Display for NewTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for NewTab {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        Self::new_tab(context, view)
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

    pub fn close_tab(context: &mut JoshutoContext, view: &JoshutoView) -> Result<(), JoshutoError> {
        if context.tabs.len() <= 1 {
            return Quit::quit(context);
        }

        let _ = context.tabs.remove(context.curr_tab_index);
        if context.curr_tab_index > 0 {
            context.curr_tab_index -= 1;
        }
        match TabSwitch::tab_switch(context.curr_tab_index, context, view) {
            Ok(_) => Ok(()),
            Err(e) => Err(JoshutoError::IO(e)),
        }
    }
}

impl JoshutoCommand for CloseTab {}

impl std::fmt::Display for CloseTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CloseTab {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        Self::close_tab(context, view)
    }
}
