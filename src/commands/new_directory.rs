use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

#[derive(Clone, Debug)]
pub struct NewDirectory {
    paths: Vec<path::PathBuf>,
}

impl NewDirectory {
    pub fn new(paths: Vec<path::PathBuf>) -> Self {
        NewDirectory { paths }
    }
    pub const fn command() -> &'static str {
        "mkdir"
    }
}

impl JoshutoCommand for NewDirectory {}

impl std::fmt::Display for NewDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for NewDirectory {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        for path in &self.paths {
            std::fs::create_dir_all(path)?;
        }
        ReloadDirList::reload(context.curr_tab_index, context)?;
        Ok(())
    }
}
