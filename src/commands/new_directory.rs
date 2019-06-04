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
        let res = ReloadDirList::reload(context.curr_tab_index, context);
        match res {
            Ok(_) => {
                let curr_tab = &mut context.tabs[context.curr_tab_index];
                curr_tab.refresh(view, &context.config_t);
                ncurses::doupdate();
            }
            Err(e) => return Err(JoshutoError::IO(e)),
        }
        Ok(())
    }
}
