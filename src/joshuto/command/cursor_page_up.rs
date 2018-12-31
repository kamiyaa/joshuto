extern crate fs_extra;
extern crate ncurses;

use std;
use std::fmt;
use std::path;

use joshuto;
use joshuto::ui;

use joshuto::command;

#[derive(Debug)]
pub struct CursorMovePageUp;

impl CursorMovePageUp {
    pub fn new() -> Self { CursorMovePageUp }
    fn command() -> &'static str { "CursorMovePageUp" }
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
