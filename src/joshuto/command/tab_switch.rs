use std;
use std::fmt;

use joshuto;
use joshuto::command;
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

    pub fn tab_switch(new_index: i32, context: &mut joshuto::JoshutoContext)
    {
        context.tab_index = new_index as usize;
        {
            let curr_tab = &mut context.tabs[context.tab_index];
            curr_tab.reload_contents(&context.config_t.sort_type);
            curr_tab.refresh(&context.views, &context.theme_t, &context.config_t,
                &context.username, &context.hostname);
        }

        ui::redraw_tab_view(&context.views.tab_win, &context);

        ncurses::doupdate();
    }
}

impl command::JoshutoCommand for TabSwitch {}

impl std::fmt::Display for TabSwitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl command::Runnable for TabSwitch {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let mut new_index = context.tab_index as i32 + self.movement;
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
