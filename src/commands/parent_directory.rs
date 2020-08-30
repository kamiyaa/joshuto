use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
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
        if context.tab_context_mut().curr_tab_mut().pwd_mut().pop() {
            let path = context.tab_context_ref().curr_tab_ref().pwd();
            std::env::set_current_dir(path)?;
        }
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
        ReloadDirList::soft_reload(context.tab_context_ref().get_index(), context)?;
        Ok(())
    }
}
