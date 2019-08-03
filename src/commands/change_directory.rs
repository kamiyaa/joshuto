use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::window::JoshutoView;

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

    pub fn change_directory(
        path: &path::PathBuf,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> std::io::Result<()> {
        std::env::set_current_dir(path.as_path())?;

        let curr_tab = &mut context.tabs[context.curr_tab_index];
        let mut curr_list = curr_tab
            .history
            .pop_or_create(&path, &context.config_t.sort_option)?;

        std::mem::swap(&mut curr_tab.curr_list, &mut curr_list);

        curr_tab
            .history
            .insert(curr_list.file_path().clone(), curr_list);
        curr_tab.curr_path = path.clone();

        curr_tab
            .history
            .populate_to_root(path, &context.config_t.sort_option)?;

        curr_tab.refresh(view, &context.config_t);
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
    fn execute(&self, context: &mut JoshutoContext, view: &JoshutoView) -> JoshutoResult<()> {
        Self::change_directory(&self.path, context, view)?;
        ncurses::doupdate();
        Ok(())
    }
}
