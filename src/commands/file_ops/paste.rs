use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::io::{IOWorkerOptions, IOWorkerThread};
use crate::ui::TuiBackend;

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
        match context.take_local_state() {
            Some(state) if !state.paths.is_empty() => {
                let dest = context.tab_context_ref().curr_tab_ref().pwd().to_path_buf();
                let mut options = self.options.clone();
                options.kind = state.file_op;
                let worker_thread = IOWorkerThread::new(options, state.paths, dest);
                context.add_worker(worker_thread);
                Ok(())
            }
            _ => Err(JoshutoError::new(
                JoshutoErrorKind::IOInvalidData,
                "no files selected".to_string(),
            )),
        }
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
