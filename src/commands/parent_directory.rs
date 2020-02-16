use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

#[derive(Clone, Debug)]
pub struct ParentDirectory;

impl ParentDirectory {
    pub fn new() -> Self {
        ParentDirectory
    }
    pub const fn command() -> &'static str {
        "parent_directory"
    }

    pub fn parent_directory(context: &mut JoshutoContext) -> std::io::Result<()> {
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        if !curr_tab.curr_path.pop() {
            return Ok(());
        }
        std::env::set_current_dir(&curr_tab.curr_path)?;
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
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        Self::parent_directory(context)?;
        Ok(())
    }
}
