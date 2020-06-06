use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::io::{IOWorkerOptions, IOWorkerThread};
use crate::ui::TuiBackend;

use super::local_state::LocalState;

pub struct PasteFiles {
    options: IOWorkerOptions,
}

impl JoshutoCommand for PasteFiles {}

impl std::fmt::Display for PasteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} overwrite={} skip_exist={}",
            Self::command(),
            self.options.overwrite,
            self.options.skip_exist,
        )
    }
}

impl std::fmt::Debug for PasteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for PasteFiles {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let state = LocalState::take();
        if state.paths.is_empty() {
            Err(JoshutoError::new(
                JoshutoErrorKind::IOInvalidData,
                "no files selected".to_string(),
            ))?;
        }
        let tab_dest = context.curr_tab_index;
        let dest = context.tabs[tab_dest].curr_path.clone();
        let mut options = self.options.clone();
        options.kind = state.file_op;
        let worker_thread = IOWorkerThread::new(options, state.paths, dest);
        context.push_worker_thread(worker_thread);
        Ok(())
    }
}

impl PasteFiles {
    pub fn new(options: IOWorkerOptions) -> Self {
        PasteFiles { options }
    }
    pub const fn command() -> &'static str {
        "paste_files"
    }
}
