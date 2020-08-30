use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::{JoshutoContext, LocalStateContext};
use crate::error::JoshutoResult;
use crate::io::FileOp;
use crate::ui::TuiBackend;

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
        if let Some(list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
            let selected = list.get_selected_paths();

            let mut local_state = LocalStateContext::new();
            local_state.set_paths(selected.into_iter());
            local_state.set_file_op(FileOp::Cut);

            context.set_local_state(local_state);
        }
        Ok(())
    }
}
