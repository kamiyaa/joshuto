use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, LoadChild};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::ui::TuiBackend;

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

        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab.curr_path = path.to_path_buf();

        Ok(())
    }

    pub fn change_directories(
        path: &path::Path,
        context: &mut JoshutoContext,
        backend: &mut TuiBackend,
    ) -> std::io::Result<()> {
        Self::cd(path, context)?;

        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab
            .history
            .populate_to_root(&path, &context.config_t.sort_option)?;

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
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        Self::change_directories(&self.path, context, backend)?;
        LoadChild::load_child(context, backend);

        Ok(())
    }
}
