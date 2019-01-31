use std;

use joshuto::command::JoshutoCommand;
use joshuto::command::JoshutoRunnable;
use joshuto::context::JoshutoContext;
use joshuto::preview;

#[derive(Clone, Debug)]
pub struct CursorMove {
    movement: i32,
}

impl CursorMove {
    pub fn new(movement: i32) -> Self
    {
        CursorMove {
            movement,
        }
    }
    pub const fn command() -> &'static str { "cursor_move" }

    pub fn cursor_move(new_index: i32, context: &mut JoshutoContext)
    {
        {
            let curr_tab = &mut context.tabs[context.curr_tab_index];

            if let Some(ref mut curr_list) = curr_tab.curr_list {
                let curr_index = curr_list.index;
                let dir_len = curr_list.contents.len() as i32;

                let mut new_index = new_index;
                if new_index <= 0 {
                    new_index = 0;
                    if curr_index <= 0 {
                        return;
                    }
                } else if new_index >= dir_len {
                    new_index = dir_len - 1;
                    if curr_index == dir_len - 1 {
                        return;
                    }
                }

                curr_list.index = new_index;
            }
            curr_tab.refresh_curr(&context.views.mid_win, context.config_t.scroll_offset);
            curr_tab.refresh_file_status(&context.views.bot_win);
            curr_tab.refresh_path_status(&context.views.top_win, &context.username, &context.hostname);
        }
        preview::preview_file(context);
        ncurses::doupdate();
    }
}

impl JoshutoCommand for CursorMove {}

impl std::fmt::Display for CursorMove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl JoshutoRunnable for CursorMove {
    fn execute(&self, context: &mut JoshutoContext)
    {
        let mut movement: Option<i32> = None;

        {
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                let curr_index = curr_list.index;
                movement = Some(curr_index + self.movement);
            }
        }
        if let Some(s) = movement {
            CursorMove::cursor_move(s, context);
        }
    }
}

#[derive(Clone, Debug)]
pub struct CursorMovePageUp;

impl CursorMovePageUp {
    pub fn new() -> Self { CursorMovePageUp }
    pub const fn command() -> &'static str { "cursor_move_page_up" }
}

impl JoshutoCommand for CursorMovePageUp {}

impl std::fmt::Display for CursorMovePageUp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for CursorMovePageUp {
    fn execute(&self, context: &mut JoshutoContext)
    {
        let mut movement: Option<i32> = None;

        {
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                let curr_index = curr_list.index;
                if curr_index <= 0 {
                    return;
                }

                let half_page = context.views.mid_win.cols / 2;
                movement = Some(curr_index - half_page);
            }
        }
        if let Some(s) = movement {
            CursorMove::cursor_move(s, context);
        }
    }
}

#[derive(Clone, Debug)]
pub struct CursorMovePageDown;

impl CursorMovePageDown {
    pub fn new() -> Self { CursorMovePageDown }
    pub const fn command() -> &'static str { "cursor_move_page_down" }
}

impl JoshutoCommand for CursorMovePageDown {}

impl std::fmt::Display for CursorMovePageDown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for CursorMovePageDown {
    fn execute(&self, context: &mut JoshutoContext)
    {
        let mut movement: Option<i32> = None;

        {
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                let curr_index = curr_list.index;
                let dir_len = curr_list.contents.len();
                if curr_index >= dir_len as i32 - 1 {
                    return;
                }

                let half_page = context.views.mid_win.cols / 2;
                movement = Some(curr_index + half_page);
            }
        }
        if let Some(s) = movement {
            CursorMove::cursor_move(s, context);
        }
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveHome;

impl CursorMoveHome {
    pub fn new() -> Self { CursorMoveHome }
    pub const fn command() -> &'static str { "cursor_move_home" }
}

impl JoshutoCommand for CursorMoveHome {}

impl std::fmt::Display for CursorMoveHome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for CursorMoveHome {
    fn execute(&self, context: &mut JoshutoContext)
    {
        let mut movement: Option<i32> = None;

        {
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                let curr_index = curr_list.index;
                if curr_index <= 0 {
                    return;
                }
                movement = Some(0);
            }
        }
        if let Some(s) = movement {
            CursorMove::cursor_move(s, context);
        }
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveEnd;

impl CursorMoveEnd {
    pub fn new() -> Self { CursorMoveEnd }
    pub const fn command() -> &'static str { "cursor_move_end" }
}

impl JoshutoCommand for CursorMoveEnd {}

impl std::fmt::Display for CursorMoveEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for CursorMoveEnd {
    fn execute(&self, context: &mut JoshutoContext)
    {
        let mut movement: Option<i32> = None;

        {
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                let curr_index = curr_list.index;
                let dir_len = curr_list.contents.len();
                if curr_index >= dir_len as i32 - 1 {
                    return;
                }
                movement = Some(dir_len as i32 - 1);
            }
        }

        if let Some(s) = movement {
            CursorMove::cursor_move(s, context);
        }
    }
}
