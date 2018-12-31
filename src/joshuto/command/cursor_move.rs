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
    fn command() -> &'static str { "CursorMove" }
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

                let new_index = curr_index + self.movement;

                let dir_len = curr_list.contents.len() as i32;
                if new_index <= 0 && curr_index == 0 ||
                        new_index >= dir_len && curr_index == dir_len - 1 {
                    return;
                }

                let dir_list = context.preview_list.take();
                if let Some(s) = dir_list {
                    context.history.insert(s);
                }

                curr_list.index = new_index;
                let curr_index = curr_list.index as usize;
                let new_path: path::PathBuf = curr_list.contents[curr_index].entry.path();

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

        if let Some(curr_list) = context.curr_list.as_mut() {

        }

        ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
        ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

        ui::redraw_status(&context.views, context.curr_list.as_ref(), &context.curr_path,
                &context.config_t.username, &context.config_t.hostname);

        ncurses::doupdate();
    }
}
