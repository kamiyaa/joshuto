use std::env;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::ui::TuiBackend;

#[derive(Clone, Debug)]
pub struct TabSwitch {
    movement: i32,
}

impl TabSwitch {
    pub fn new(movement: i32) -> Self {
        TabSwitch { movement }
    }
    pub const fn command() -> &'static str {
        "tab_switch"
    }

    pub fn tab_switch(new_index: usize, context: &mut JoshutoContext) -> std::io::Result<()> {
        context.tab_context_mut().set_index(new_index);
        let path = context.tab_context_ref().curr_tab_ref().pwd().to_path_buf();
        env::set_current_dir(path.as_path())?;

        let options = context.config_t.sort_option.clone();
        context
            .tab_context_mut()
            .curr_tab_mut()
            .history_mut()
            .create_or_soft_update(path.as_path(), &options);
        Ok(())
    }
}

impl JoshutoCommand for TabSwitch {}

impl std::fmt::Display for TabSwitch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl JoshutoRunnable for TabSwitch {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        let new_index = (context.tab_context_ref().get_index() as i32 + self.movement)
            % context.tab_context_ref().len() as i32;
        let new_index = new_index as usize;
        Self::tab_switch(new_index, context)?;
        Ok(())
    }
}
