extern crate fs_extra;
extern crate ncurses;

use std;
use std::fmt;
use std::path;
use std::process;

use joshuto;
use joshuto::command;
use joshuto::ui;

#[derive(Debug)]
pub struct ChangeDirectory {
    path: path::PathBuf,
}

impl ChangeDirectory {
    pub fn new(path: path::PathBuf) -> Self
    {
        ChangeDirectory {
            path,
        }
    }
    pub const fn command() -> &'static str { "change_directory" }
}

impl command::JoshutoCommand for ChangeDirectory {}

impl std::fmt::Display for ChangeDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} {}", Self::command(), self.path.to_str().unwrap())
    }
}

impl command::Runnable for ChangeDirectory {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if !self.path.exists() {
            ui::wprint_err(&context.views.bot_win, "Error: No such file or directory");
            ncurses::doupdate();
            return;
        }

        context.curr_path = self.path.clone();

        {
            context.history.populate_to_root(&context.curr_path, &context.config_t.sort_type);

            let parent_list = context.parent_list.take();
            context.history.put_back(parent_list);

            let curr_list = context.curr_list.take();
            context.history.put_back(curr_list);

            let preview_list = context.preview_list.take();
            context.history.put_back(preview_list);
        }

        context.curr_list = match context.history.pop_or_create(&context.curr_path,
                    &context.config_t.sort_type) {
            Ok(s) => {
                if let Some(dirent) = s.get_curr_entry() {
                    if dirent.path.is_dir() {
                        context.preview_list = match context.history.pop_or_create(
                                    &dirent.path, &context.config_t.sort_type) {
                            Ok(s) => {
                                Some(s)
                            },
                            Err(e) => {
                                eprintln!("{}", e);
                                None
                            },
                        };
                    }
                }
                Some(s)
            },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

        if let Some(parent) = context.curr_path.parent() {
            context.parent_list = match context.history.pop_or_create(&parent, &context.config_t.sort_type) {
                Ok(s) => { Some(s) },
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                },
            };
        }

        ui::redraw_view(&context.views.left_win, context.parent_list.as_ref());
        ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
        ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

        ui::redraw_status(&context.views, context.curr_list.as_ref(), &context.curr_path,
                &context.config_t.username, &context.config_t.hostname);

        ncurses::doupdate();
    }
}
