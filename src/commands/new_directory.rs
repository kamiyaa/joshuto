use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::window::JoshutoView;

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
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        for path in &self.paths {
            match std::fs::create_dir_all(path) {
                Ok(_) => {}
                Err(e) => return Err(JoshutoError::IO(e)),
            }
        }
        match ReloadDirList::reload(context, view) {
            Ok(_) => {}
            Err(e) => return Err(JoshutoError::IO(e)),
        }
        ncurses::doupdate();
        Ok(())
    }
}
