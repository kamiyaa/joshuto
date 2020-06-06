use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::io::FileOp;
use crate::ui::TuiBackend;

use super::local_state::LocalState;

#[derive(Clone, Debug)]
pub struct CutFiles;

impl CutFiles {
    pub fn new() -> Self {
        CutFiles
    }
    pub const fn command() -> &'static str {
        "cut_files"
    }
}

impl JoshutoCommand for CutFiles {}

impl std::fmt::Display for CutFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CutFiles {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let curr_tab = context.curr_tab_ref();
        if let Some(list) = curr_tab.curr_list_ref() {
            LocalState::repopulate(list)?;
            LocalState::set_file_op(FileOp::Cut);
        }
        Ok(())
    }
}
