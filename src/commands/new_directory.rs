use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub struct NewDirectory {
    path: path::PathBuf,
}

impl NewDirectory {
    pub fn new(path: path::PathBuf) -> Self {
        NewDirectory { path }
    }
    pub const fn command() -> &'static str {
        "mkdir"
    }

    fn new_directory(
        context: &mut JoshutoContext,
        view: &JoshutoView,
        path: &path::Path,
    ) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(path)?;
        ReloadDirList::reload(context, view)?;
        ncurses::doupdate();
        Ok(())
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
        match Self::new_directory(context, view, self.path.as_path()) {
            Ok(_) => Ok(()),
            Err(e) => Err(JoshutoError::IO(e)),
        }
    }
}
