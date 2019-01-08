extern crate fs_extra;
extern crate ncurses;

use std;
use std::fmt;

use joshuto;
use joshuto::ui;

use joshuto::command;

#[derive(Debug)]
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
    pub fn command() -> &'static str { "cursor_move" }

    pub fn cursor_move(new_index: i32, context: &mut joshuto::JoshutoContext)
    {
        let curr_tab = &mut context.tabs[context.tab_index];

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

            let dir_list = curr_tab.preview_list.take();
            if let Some(s) = dir_list {
                curr_tab.history.insert(s);
            }

            curr_list.index = new_index;
        }

        if let Some(ref curr_list) = curr_tab.curr_list {
            let curr_index = curr_list.index as usize;
            let new_path = &curr_list.contents[curr_index].path;

            curr_list.display_contents(&context.views.mid_win);
            ncurses::wnoutrefresh(context.views.mid_win.win);

            if new_path.is_dir() {
                match curr_tab.history.pop_or_create(new_path.as_path(),
                        &context.config_t.sort_type) {
                    Ok(s) => {
                        curr_tab.preview_list = Some(s);
                        ui::redraw_view(&context.views.right_win, curr_tab.preview_list.as_ref());
                    },
                    Err(e) => ui::wprint_err(&context.views.right_win, e.to_string().as_str()),
                }
            } else {
                ncurses::werase(context.views.right_win.win);
                ncurses::wnoutrefresh(context.views.right_win.win);
            }

            ui::redraw_status(&context.views, curr_tab.curr_list.as_ref(), &curr_tab.curr_path,
                    &context.username, &context.hostname);

            ncurses::doupdate();
        }
    }
}

impl command::JoshutoCommand for CursorMove {}

impl std::fmt::Display for CursorMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl command::Runnable for CursorMove {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let mut movement: Option<i32> = None;

        {
            let curr_tab = &mut context.tabs[context.tab_index];
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

#[derive(Debug)]
pub struct CursorMovePageUp;

impl CursorMovePageUp {
    pub fn new() -> Self { CursorMovePageUp }
    pub fn command() -> &'static str { "cursor_move_page_up" }
}

impl command::JoshutoCommand for CursorMovePageUp {}

impl std::fmt::Display for CursorMovePageUp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for CursorMovePageUp {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let mut movement: Option<i32> = None;

        {
            let curr_tab = &mut context.tabs[context.tab_index];
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

#[derive(Debug)]
pub struct CursorMovePageDown;

impl CursorMovePageDown {
    pub fn new() -> Self { CursorMovePageDown }
    pub fn command() -> &'static str { "cursor_move_page_down" }
}

impl command::JoshutoCommand for CursorMovePageDown {}

impl std::fmt::Display for CursorMovePageDown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for CursorMovePageDown {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let mut movement: Option<i32> = None;

        {
            let curr_tab = &mut context.tabs[context.tab_index];
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

#[derive(Debug)]
pub struct CursorMoveHome;

impl CursorMoveHome {
    pub fn new() -> Self { CursorMoveHome }
    pub fn command() -> &'static str { "cursor_move_home" }
}

impl command::JoshutoCommand for CursorMoveHome {}

impl std::fmt::Display for CursorMoveHome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for CursorMoveHome {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let mut movement: Option<i32> = None;

        {
            let curr_tab = &mut context.tabs[context.tab_index];
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

#[derive(Debug)]
pub struct CursorMoveEnd;

impl CursorMoveEnd {
    pub fn new() -> Self { CursorMoveEnd }
    pub fn command() -> &'static str { "cursor_move_end" }
}

impl command::JoshutoCommand for CursorMoveEnd {}

impl std::fmt::Display for CursorMoveEnd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for CursorMoveEnd {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let mut movement: Option<i32> = None;

        {
            let curr_tab = &mut context.tabs[context.tab_index];
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
