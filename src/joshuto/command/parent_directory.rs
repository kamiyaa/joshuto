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
    pub fn command() -> &'static str { "parent_directory" }
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
        let curr_tab = &mut context.tabs[context.tab_index];

        if curr_tab.curr_path.pop() == false {
            return;
        }

        match std::env::set_current_dir(&curr_tab.curr_path) {
            Ok(_) => {
                {
                    let dir_list = curr_tab.preview_list.take();
                    curr_tab.history.put_back(dir_list);

                    let curr_list = curr_tab.curr_list.take();
                    curr_tab.preview_list = curr_list;

                    let parent_list = curr_tab.parent_list.take();
                    curr_tab.curr_list = parent_list;
                }

                match curr_tab.curr_path.parent() {
                    Some(parent) => {
                        curr_tab.parent_list = match curr_tab.history.pop_or_create(&parent, &context.config_t.sort_type) {
                            Ok(s) => {
                                s.display_contents(&context.views.left_win);
                                Some(s)
                            },
                            Err(e) => {
                                ui::wprint_err(&context.views.left_win, e.to_string().as_str());
                                None
                            },
                        };
                    },
                    None => {
                        ncurses::werase(context.views.left_win.win);
                        ncurses::wnoutrefresh(context.views.left_win.win);
                    },
                }
                ui::redraw_view(&context.views.left_win, curr_tab.parent_list.as_ref());
                ui::redraw_view(&context.views.mid_win, curr_tab.curr_list.as_ref());
                ui::redraw_view(&context.views.right_win, curr_tab.preview_list.as_ref());

                ui::redraw_status(&context.views, curr_tab.curr_list.as_ref(), &curr_tab.curr_path,
                        &context.username, &&context.hostname);
            },
            Err(e) => {
                ui::wprint_err(&context.views.bot_win, e.to_string().as_str());
            },
        };

        ncurses::doupdate();
    }
}
