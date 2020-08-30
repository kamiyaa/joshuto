use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;

#[derive(Clone, Debug)]
pub struct ChangeDirectory {
    path: path::PathBuf,
}

impl ChangeDirectory {
    pub fn new(path: path::PathBuf) -> Self {
        ChangeDirectory { path }
    }
    pub const fn command() -> &'static str {
        "cd"
    }

    pub fn cd(path: &path::Path, context: &mut JoshutoContext) -> std::io::Result<()> {
        std::env::set_current_dir(path)?;
        context.tab_context_mut().curr_tab_mut().set_pwd(path);
        Ok(())
    }

    pub fn change_directories(
        path: &path::Path,
        context: &mut JoshutoContext,
    ) -> std::io::Result<()> {
        Self::cd(path, context)?;

        let sort_options = context.config_t.sort_option.clone();
        context
            .tab_context_mut()
            .curr_tab_mut()
            .history
            .populate_to_root(&path, &sort_options)?;

        Ok(())
    }
}

impl JoshutoCommand for ChangeDirectory {}

impl std::fmt::Display for ChangeDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.path.to_str().unwrap())
    }
}

impl JoshutoRunnable for ChangeDirectory {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        Self::change_directories(&self.path, context)?;
        LoadChild::load_child(context)?;

        Ok(())
    }
}
