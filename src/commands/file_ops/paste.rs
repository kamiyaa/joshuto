use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::io::Options;
use crate::ui::TuiBackend;

use super::local_state::{FileOp, LocalState};
use super::paste_copy::paste_copy;
use super::paste_cut::paste_cut;

pub struct PasteFiles {
    options: Options,
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
        let file_operation = LocalState::get_file_operation();
        let thread = match file_operation {
            FileOp::Copy => paste_copy(context, self.options.clone()),
            FileOp::Cut => paste_cut(context, self.options.clone()),
        };
        let thread = thread?;
        context.add_new_worker(thread);
        Ok(())
    }
}

impl PasteFiles {
    pub fn new(options: Options) -> Self {
        PasteFiles { options }
    }
    pub const fn command() -> &'static str {
        "paste_files"
    }
}
