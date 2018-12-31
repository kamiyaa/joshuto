extern crate fs_extra;
extern crate ncurses;

use std;

use std::fmt;

use joshuto;
use joshuto::ui;
use joshuto::command;

#[derive(Debug)]
pub struct ParentDirectory;

impl ParentDirectory {
    pub fn new() -> Self { ParentDirectory }
    fn command() -> &'static str { "ParentDirectory" }
}

impl command::JoshutoCommand for ParentDirectory {}

impl std::fmt::Display for ParentDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for ParentDirectory {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if context.curr_path.pop() == false {
            return;
        }

        match std::env::set_current_dir(context.curr_path.as_path()) {
            Ok(_) => {
                {
                    let dir_list = context.preview_list.take();
                    context.history.put_back(dir_list);

                    let curr_list = context.curr_list.take();
                    context.preview_list = curr_list;

                    let parent_list = context.parent_list.take();
                    context.curr_list = parent_list;
                }

                match context.curr_path.parent() {
                    Some(parent) => {
                        context.parent_list = match context.history.pop_or_create(&parent, &context.config_t.sort_type) {
                            Ok(s) => {
                                s.display_contents(&context.views.left_win);
                                Some(s)
                            },
                            Err(e) => {
                                ui::wprint_err(&context.views.left_win, format!("{}", e).as_str());
                                None
                            },
                        };
                    },
                    None => {
                        ncurses::werase(context.views.left_win.win);
                        ncurses::wnoutrefresh(context.views.left_win.win);
                    },
                }
                ui::redraw_view(&context.views.left_win, context.parent_list.as_ref());
                ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
                ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

                ui::redraw_status(&context.views, context.curr_list.as_ref(), &context.curr_path,
                        &context.config_t.username, &&context.config_t.hostname);
            },
            Err(e) => {
                ui::wprint_err(&context.views.bot_win, format!("{}", e).as_str());
            },
        };

        ncurses::doupdate();
    }
}
