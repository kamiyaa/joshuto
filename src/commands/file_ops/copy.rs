use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::{JoshutoContext, LocalStateContext};
use crate::error::JoshutoResult;
use crate::io::FileOp;
use crate::ui::TuiBackend;

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
        if let Some(list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
            let mut selected: Vec<&path::Path> = list
                .iter()
                .filter(|e| e.is_selected())
                .map(|e| e.file_path())
                .collect();
            if selected.is_empty() {
                selected = match list.get_curr_ref() {
                    Some(s) => vec![s.file_path()],
                    None => vec![],
                }
            }

            let mut local_state = LocalStateContext::new();
            local_state.set_paths(selected.into_iter());
            local_state.set_file_op(FileOp::Copy);

            context.set_local_state(local_state);
        }
        Ok(())
    }
}
