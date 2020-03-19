use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

use crate::sort::SortType;

use crate::HOME_DIR;

#[derive(Clone, Debug)]
pub struct Sort {
    sort_method: SortType,
}

impl Sort {
    pub fn new(sort_method: SortType) -> Self {
        Self { sort_method }
    }
    pub const fn command() -> &'static str {
        "sort"
    }
}

impl JoshutoCommand for Sort {}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for Sort {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        context.config_t.sort_option.sort_method = self.sort_method;
        Ok(())
    }
}
