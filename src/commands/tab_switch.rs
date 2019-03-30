use std::env;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::preview;
use crate::ui;
use crate::window::JoshutoView;

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

    pub fn tab_switch(new_index: i32, context: &mut JoshutoContext, view: &JoshutoView) {
        context.curr_tab_index = new_index as usize;
        let path = &context.curr_tab_ref().curr_path;
        match env::set_current_dir(path) {
            Ok(_) => {
                {
                    let curr_tab = &mut context.tabs[context.curr_tab_index];
                    curr_tab.reload_contents(&context.config_t.sort_type);
                    curr_tab.refresh(
                        view,
                        &context.config_t,
                        &context.username,
                        &context.hostname,
                    );
                }
                ui::redraw_tab_view(&view.tab_win, &context);
                let curr_tab = &mut context.tabs[context.curr_tab_index];
                preview::preview_file(curr_tab, view, &context.config_t);
            }
            Err(e) => ui::wprint_err(&view.left_win, e.to_string().as_str()),
        }
    }
}

impl JoshutoCommand for TabSwitch {}

impl std::fmt::Display for TabSwitch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl JoshutoRunnable for TabSwitch {
    fn execute(&self, context: &mut JoshutoContext, view: &JoshutoView) {
        let mut new_index = context.curr_tab_index as i32 + self.movement;
        let tab_len = context.tabs.len() as i32;
        while new_index < 0 {
            new_index += tab_len;
        }
        while new_index >= tab_len {
            new_index -= tab_len;
        }
        Self::tab_switch(new_index, context, view);
        ncurses::doupdate();
    }
}
