use std::env;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
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

    pub fn tab_switch(
        new_index: usize,
        context: &mut JoshutoContext,
        backend: &mut TuiBackend,
    ) -> std::io::Result<()> {
        context.curr_tab_index = new_index;
        let path = &context.curr_tab_ref().curr_path;
        env::set_current_dir(path)?;

        /*
                ui::redraw_tab_view(&view.tab_win, &context);
                ncurses::doupdate();
        */
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
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        let mut new_index = context.curr_tab_index as i32 + self.movement;
        let tab_len = context.tabs.len() as i32;
        while new_index < 0 {
            new_index += tab_len;
        }
        while new_index >= tab_len {
            new_index -= tab_len;
        }
        let new_index = new_index as usize;
        Self::tab_switch(new_index, context, backend)?;
        Ok(())
    }
}
