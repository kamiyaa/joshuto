extern crate fs_extra;
extern crate ncurses;

use std;
use std::fmt;
use std::path;

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
        match context.curr_list {
            Some(ref mut curr_list) => {
                let curr_index = curr_list.index;

                let new_index = curr_list.index + self.movement;

                let dir_len = curr_list.contents.len() as i32;
                if new_index <= 0 && curr_index <= 0 ||
                        new_index >= dir_len && curr_index == dir_len - 1 {
                    return;
                }

                let dir_list = context.preview_list.take();
                if let Some(s) = dir_list {
                    context.history.insert(s);
                }

                curr_list.index = new_index;
                let curr_index = curr_list.index as usize;
                let new_path = &curr_list.contents[curr_index].path;

                if new_path.is_dir() {
                    match context.history.pop_or_create(new_path.as_path(),
                            &context.config_t.sort_type) {
                        Ok(s) => context.preview_list = Some(s),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            },
            None => {},
        }

        ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
        ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

        ui::redraw_status(&context.views, context.curr_list.as_ref(), &context.curr_path,
                &context.config_t.username, &context.config_t.hostname);

        ncurses::doupdate();
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
        match context.curr_list {
            Some(ref mut curr_list) => {
                let curr_index = curr_list.index;

                if curr_index <= 0 {
                    return;
                }

                let half_page = context.views.mid_win.cols as usize / 2;

                let new_index = if curr_index <= half_page as i32 {
                    0
                } else {
                    curr_index - half_page as i32
                };

                let dir_list = context.preview_list.take();
                if let Some(s) = dir_list {
                    context.history.insert(s);
                }

                curr_list.index = new_index;
                let curr_index = curr_list.index as usize;

                if curr_list.contents[curr_index].path.is_dir() {
                    match context.history.pop_or_create(&curr_list.contents[curr_index].path,
                            &context.config_t.sort_type) {
                        Ok(s) => context.preview_list = Some(s),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            },
            None => {},
        }

        ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
        ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

        ui::redraw_status(&context.views, context.curr_list.as_ref(), &context.curr_path,
                &context.config_t.username, &context.config_t.hostname);
        ncurses::doupdate();
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
        match context.curr_list {
            Some(ref mut curr_list) => {
                let curr_index = curr_list.index as usize;
                let dir_len = curr_list.contents.len();

                if curr_index >= dir_len - 1 {
                    return;
                }

                let half_page = context.views.mid_win.cols as usize / 2;

                let new_index = if curr_index as usize + half_page >= dir_len {
                    (dir_len - 1)
                } else {
                    curr_index as usize + half_page
                };

                let dir_list = context.preview_list.take();
                if let Some(s) = dir_list {
                    context.history.insert(s);
                }

                curr_list.index = new_index as i32;
                let new_path = &curr_list.contents[new_index].path;

                if new_path.is_dir() {
                    match context.history.pop_or_create(new_path.as_path(),
                            &context.config_t.sort_type) {
                        Ok(s) => context.preview_list = Some(s),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            },
            None => {},
        }

        ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
        ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

        ui::redraw_status(&context.views, context.curr_list.as_ref(), &context.curr_path,
                &context.config_t.username, &context.config_t.hostname);
        ncurses::doupdate();
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

        match context.curr_list {
            Some(ref mut curr_list) => {
                if curr_list.index <= 0 {
                    return;
                }

                let dir_list = context.preview_list.take();
                if let Some(s) = dir_list {
                    context.history.insert(s);
                }

                curr_list.index = 0;
                let new_path = &curr_list.contents[curr_list.index as usize].path;

                if new_path.is_dir() {
                    match context.history.pop_or_create(new_path.as_path(),
                            &context.config_t.sort_type) {
                        Ok(s) => context.preview_list = Some(s),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            },
            None => {},
        }

        ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
        ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

        ui::redraw_status(&context.views, context.curr_list.as_ref(), &context.curr_path,
                &context.config_t.username, &context.config_t.hostname);

        ncurses::doupdate();
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
        match context.curr_list {
            Some(ref mut curr_list) => {
                let dir_len = curr_list.contents.len();

                if curr_list.index >= dir_len as i32 - 1 {
                    return;
                }

                let dir_list = context.preview_list.take();
                if let Some(s) = dir_list {
                    context.history.insert(s);
                }

                curr_list.index = dir_len as i32 - 1;

                if curr_list.contents[curr_list.index as usize].path.is_dir() {
                    match context.history.pop_or_create(&curr_list.contents[curr_list.index as usize].path,
                            &context.config_t.sort_type) {
                        Ok(s) => context.preview_list = Some(s),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            },
            None => {},
        }

        ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
        ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

        ui::redraw_status(&context.views, context.curr_list.as_ref(), &context.curr_path,
                &context.config_t.username, &context.config_t.hostname);

        ncurses::doupdate();
    }
}
