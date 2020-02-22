use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

#[derive(Clone, Debug)]
pub struct ReloadDirList;

impl ReloadDirList {
    pub fn new() -> Self {
        ReloadDirList
    }
    pub const fn command() -> &'static str {
        "reload_dir_list"
    }

    pub fn reload(index: usize, context: &mut JoshutoContext) -> std::io::Result<()> {
        let curr_tab = &mut context.tabs[index];
        let sort_option = &context.config_t.sort_option;

        match curr_tab.curr_list_mut() {
            Some(curr_list) => curr_list.reload_contents(sort_option)?,
            None => {}
        }
        match curr_tab.parent_list_mut() {
            Some(curr_list) => curr_list.reload_contents(sort_option)?,
            None => {}
        }
        match curr_tab.child_list_mut() {
            Some(curr_list) => curr_list.reload_contents(sort_option)?,
            None => {}
        }

        Ok(())
    }
}

impl JoshutoCommand for ReloadDirList {}

impl std::fmt::Display for ReloadDirList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for ReloadDirList {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        Self::reload(context.curr_tab_index, context)?;
        Ok(())
    }
}
