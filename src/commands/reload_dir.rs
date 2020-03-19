use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;

#[derive(Clone, Debug)]
pub struct ReloadDirList;

impl ReloadDirList {
    pub fn new() -> Self {
        ReloadDirList
    }
    pub const fn command() -> &'static str {
        "reload_dir_list"
    }

    pub fn soft_reload(index: usize, context: &mut JoshutoContext) -> std::io::Result<()> {
        let curr_tab = &mut context.tabs[index];
        let sort_option = &context.config_t.sort_option;

        if let Some(curr_list) = curr_tab.curr_list_mut() {
            if curr_list.need_update() {
                curr_list.reload_contents(sort_option)?;
            }
        }
        if let Some(curr_list) = curr_tab.parent_list_mut() {
            if curr_list.need_update() {
                curr_list.reload_contents(sort_option)?;
            }
        }
        if let Some(curr_list) = curr_tab.child_list_mut() {
            if curr_list.need_update() {
                curr_list.reload_contents(sort_option)?;
            }
        }

        Ok(())
    }

    pub fn reload(index: usize, context: &mut JoshutoContext) -> std::io::Result<()> {
        let curr_tab = &mut context.tabs[index];
        let sort_option = &context.config_t.sort_option;

        if let Some(curr_list) = curr_tab.curr_list_mut() {
            curr_list.reload_contents(sort_option)?;
        }
        if let Some(curr_list) = curr_tab.parent_list_mut() {
            curr_list.reload_contents(sort_option)?;
        }
        if let Some(curr_list) = curr_tab.child_list_mut() {
            curr_list.reload_contents(sort_option)?;
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
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        Self::reload(context.curr_tab_index, context)?;
        LoadChild::load_child(context)?;
        Ok(())
    }
}
