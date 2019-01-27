use std;

use joshuto::context::JoshutoContext;
use joshuto::command::JoshutoCommand;
use joshuto::command::JoshutoRunnable;
use joshuto::ui;

#[derive(Clone, Debug)]
pub struct TabSwitch {
    movement: i32,
}

impl TabSwitch {
    pub fn new(movement: i32) -> Self {
        TabSwitch {
            movement,
        }
    }
    pub const fn command() -> &'static str { "tab_switch" }

    pub fn tab_switch(new_index: i32, context: &mut JoshutoContext)
    {
        context.curr_tab_index = new_index as usize;
        {
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            curr_tab.reload_contents(&context.config_t.sort_type);
            curr_tab.refresh(&context.views, &context.config_t,
                &context.username, &context.hostname);
        }

        ui::redraw_tab_view(&context.views.tab_win, &context);

        ncurses::doupdate();
    }
}

impl JoshutoCommand for TabSwitch {}

impl std::fmt::Display for TabSwitch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl JoshutoRunnable for TabSwitch {
    fn execute(&self, context: &mut JoshutoContext)
    {
        let mut new_index = context.curr_tab_index as i32 + self.movement;
        let tab_len = context.tabs.len() as i32;
        while new_index < 0 {
            new_index = new_index + tab_len;
        }
        while new_index >= tab_len {
            new_index = new_index - tab_len;
        }
        Self::tab_switch(new_index, context);
    }
}
