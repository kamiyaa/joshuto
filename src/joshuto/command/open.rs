extern crate fs_extra;
extern crate ncurses;

use std;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::mem;
use std::path;

use joshuto;
use joshuto::command;
use joshuto::structs;
use joshuto::ui;
use joshuto::unix;

#[derive(Debug)]
pub struct Open;

impl Open {
    pub fn new() -> Self { Open }
    pub fn command() -> &'static str { "Open" }
}

impl command::JoshutoCommand for Open {}

impl std::fmt::Display for Open {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for Open {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let index: usize;
        let path: path::PathBuf;
        if let Some(s) = context.curr_list.as_ref() {
            if s.contents.len() == 0 {
                return;
            } else {
                index = s.index as usize;
                path = s.contents[index].entry.path();
            }
        } else {
            return;
        }

        if path.is_file() {
            unix::open_file(&context.mimetype_t.mimetypes, &context.views.bot_win, path.as_path());
        } else if path.is_dir() {
            match env::set_current_dir(&path) {
                Ok(_) => {},
                Err(e) => {
                    ui::wprint_err(&context.views.bot_win, format!("{}: {:?}", e, path).as_str());
                    return;
                }
            }

            {
                let dir_list = context.parent_list.take();
                context.history.put_back(dir_list);

                let curr_list = context.curr_list.take();
                context.parent_list = curr_list;

                let preview_list = context.preview_list.take();
                context.curr_list = preview_list;
            }

            /* update curr_path */
            match path.strip_prefix(context.curr_path.as_path()) {
                Ok(s) => context.curr_path.push(s),
                Err(e) => {
                    ui::wprint_err(&context.views.bot_win, format!("{}", e).as_str());
                    return;
                }
            }

            if let Some(s) = context.curr_list.as_ref() {
                if s.contents.len() > 0 {
                    let dirent: &structs::JoshutoDirEntry = &s.contents[s.index as usize];
                    let new_path: path::PathBuf = dirent.entry.path();

                    if new_path.is_dir() {
                        context.preview_list = match context.history.pop_or_create(
                                    new_path.as_path(), &context.config_t.sort_type) {
                            Ok(s) => { Some(s) },
                            Err(e) => {
                                ui::wprint_err(&context.views.right_win,
                                        format!("{}", e).as_str());
                                None
                            },
                        };
                    } else {
                        ncurses::werase(context.views.right_win.win);
                    }
                }
            }

            ui::redraw_view(&context.views.left_win, context.parent_list.as_ref());
            ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
            ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

            ui::redraw_status(&context.views, context.curr_list.as_ref(), &context.curr_path,
                    &context.config_t.username, &context.config_t.hostname);

            ncurses::doupdate();
        }
    }
}
