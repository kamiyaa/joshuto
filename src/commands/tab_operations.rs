use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, Quit, TabSwitch};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::tab::JoshutoTab;
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;

use crate::HOME_DIR;

#[derive(Clone, Debug)]
pub struct NewTab;

impl NewTab {
    pub fn new() -> Self {
        NewTab
    }
    pub const fn command() -> &'static str {
        "new_tab"
    }

    pub fn new_tab(context: &mut JoshutoContext) -> JoshutoResult<()> {
        /* start the new tab in $HOME or root */
        let curr_path = match HOME_DIR.as_ref() {
            Some(s) => s.clone(),
            None => path::PathBuf::from("/"),
        };

        let tab = JoshutoTab::new(curr_path, &context.config_t.sort_option)?;
        context.tab_context_mut().push_tab(tab);
        let new_index = context.tab_context_ref().len() - 1;
        context.tab_context_mut().set_index(new_index);
        TabSwitch::tab_switch(new_index, context)?;
        LoadChild::load_child(context)?;
        Ok(())
    }
}

impl JoshutoCommand for NewTab {}

impl std::fmt::Display for NewTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for NewTab {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        Self::new_tab(context)
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

    pub fn close_tab(context: &mut JoshutoContext) -> JoshutoResult<()> {
        if context.tab_context_ref().len() <= 1 {
            return Quit::quit(context);
        }
        let mut tab_index = context.tab_context_ref().get_index();

        let _ = context.tab_context_mut().pop_tab(tab_index);
        if tab_index > 0 {
            tab_index -= 1;
            context.tab_context_mut().set_index(tab_index);
        }
        TabSwitch::tab_switch(tab_index, context)?;
        Ok(())
    }
}

impl JoshutoCommand for CloseTab {}

impl std::fmt::Display for CloseTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CloseTab {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        Self::close_tab(context)
    }
}
