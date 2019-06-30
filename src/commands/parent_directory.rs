use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoResult};
use crate::history::DirectoryHistory;
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub struct ParentDirectory;

impl ParentDirectory {
    pub fn new() -> Self {
        ParentDirectory
    }
    pub const fn command() -> &'static str {
        "parent_directory"
    }

    pub fn parent_directory(
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> std::io::Result<()> {
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        if !curr_tab.curr_path.pop() {
            return Ok(());
        }
        std::env::set_current_dir(&curr_tab.curr_path)?;

        let mut new_curr_list = curr_tab
            .history
            .pop_or_create(&curr_tab.curr_path, &context.config_t.sort_option)?;

        std::mem::swap(&mut curr_tab.curr_list, &mut new_curr_list);
        curr_tab
            .history
            .insert(new_curr_list.file_path().clone(), new_curr_list);

        curr_tab.refresh(view, &context.config_t);
        ncurses::doupdate();
        Ok(())
    }
}

impl JoshutoCommand for ParentDirectory {}

impl std::fmt::Display for ParentDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for ParentDirectory {
    fn execute(&self, context: &mut JoshutoContext, view: &JoshutoView) -> JoshutoResult<()> {
        match Self::parent_directory(context, view) {
            Ok(_) => Ok(()),
            Err(e) => Err(JoshutoError::from(e)),
        }
    }
}
