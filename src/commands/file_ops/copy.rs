use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::io::FileOp;
use crate::ui::TuiBackend;

use super::local_state::LocalState;

#[derive(Clone, Debug)]
pub struct CopyFiles;

impl CopyFiles {
    pub fn new() -> Self {
        CopyFiles
    }
    pub const fn command() -> &'static str {
        "copy_files"
    }
}

impl JoshutoCommand for CopyFiles {}

impl std::fmt::Display for CopyFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CopyFiles {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let curr_tab = context.curr_tab_ref();
        if let Some(list) = curr_tab.curr_list_ref() {
            LocalState::repopulate(list)?;
            LocalState::set_file_op(FileOp::Copy);
        }
        Ok(())
    }
}
