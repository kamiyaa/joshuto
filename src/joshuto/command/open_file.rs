extern crate fs_extra;
extern crate ncurses;
extern crate mime_guess;

use std;
use std::env;
use std::fmt;
use std::path;

use joshuto;
use joshuto::command;
use joshuto::mimetype;
use joshuto::structs;
use joshuto::ui;
use joshuto::unix;
use joshuto::window;

#[derive(Debug)]
pub struct OpenFile;

impl OpenFile {
    pub fn new() -> Self { OpenFile }
    pub fn command() -> &'static str { "open_file" }
}

impl command::JoshutoCommand for OpenFile {}

impl std::fmt::Display for OpenFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for OpenFile {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let index: usize;
        let path: path::PathBuf;
        if let Some(s) = context.curr_list.as_ref() {
            if s.contents.len() == 0 {
                return;
            } else {
                index = s.index as usize;
                path = s.contents[index].path.clone();
            }
        } else {
            return;
        }

        if path.is_file() {
            let file_ext: Option<&str> = match path.extension() {
                Some(s) => s.to_str(),
                None => None,
                };

            let mimetype: Option<&str> = match file_ext {
                Some(extstr) => mime_guess::get_mime_type_str(extstr),
                None => None,
                };

            let empty_vec: Vec<Vec<String>> = Vec::new();
            let mimetype_options: &Vec<Vec<String>> = match mimetype {
                    Some(mimetype) => {
                        match context.mimetype_t.mimetypes.get(mimetype) {
                            Some(s) => s,
                            None => &empty_vec,
                        }
                    },
                    None => {
                        &empty_vec
                    }
                };

            if mimetype_options.len() > 0 {
                ncurses::savetty();
                ncurses::endwin();
                unix::open_with(path.as_path(), &mimetype_options[0]);
                ncurses::resetty();
                ncurses::refresh();
            } else {
                match mimetype {
                    Some(s) => ui::wprint_err(&context.views.bot_win,
                                format!("Don't know how to open: {}", s).as_str()),
                    None => ui::wprint_err(&context.views.bot_win,
                                "Uh oh, mime_guess says unknown file type :("),
                };
            }
            ncurses::doupdate();

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
                    let new_path: path::PathBuf = dirent.path.clone();

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

#[derive(Debug)]
pub struct OpenFileWith;

impl OpenFileWith {
    pub fn new() -> Self { OpenFileWith }
    pub fn command() -> &'static str { "open_file_with" }

    pub fn open_with(pathbuf: path::PathBuf, mimetype_t: &mimetype::JoshutoMimetype)
    {
        let mut term_rows: i32 = 0;
        let mut term_cols: i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

        let file_ext: Option<&str> = match pathbuf.extension() {
            Some(s) => s.to_str(),
            None => None,
            };

        let mimetype: Option<&str> = match file_ext {
            Some(extstr) => mime_guess::get_mime_type_str(extstr),
            None => None,
            };

        let empty_vec: Vec<Vec<String>> = Vec::new();
        let mimetype_options: &Vec<Vec<String>> = match mimetype {
            Some(mimetype) => {
                match mimetype_t.mimetypes.get(mimetype) {
                    Some(s) => s,
                    None => &empty_vec,
                }
            },
            None => &empty_vec,
            };

        let option_size = mimetype_options.len();
        let mut win = window::JoshutoPanel::new(option_size as i32 + 2, term_cols,
                (term_rows as usize - option_size - 2, 0));
        ncurses::keypad(win.win, true);

        let mut display_vec: Vec<String> = Vec::with_capacity(option_size);
        for (i, val) in mimetype_options.iter().enumerate() {
            display_vec.push(format!("  {}\t{}", i, val.join(" ")));
        }
        display_vec.sort();

        win.move_to_top();
        ui::display_options(&win, &display_vec);
        ncurses::doupdate();

        ncurses::wmove(win.win, option_size as i32 + 1, 0);
        const PROMPT: &str = ":open_with ";
        ncurses::waddstr(win.win, PROMPT);

        match ui::get_str(&win, (option_size as i32 + 1, PROMPT.len() as i32)) {
            Some(user_input) => {
                win.destroy();
                ncurses::update_panels();
                ncurses::doupdate();
                match user_input.parse::<usize>() {
                    Ok(s) => {
                        if s < mimetype_options.len() {
                            ncurses::savetty();
                            ncurses::endwin();
                            unix::open_with(pathbuf.as_path(), &mimetype_options[s]);
                            ncurses::resetty();
                            ncurses::refresh();
                        }
                    }
                    Err(_) => {
                        let args: Vec<String> = user_input.split_whitespace().map(|x| String::from(x)).collect();
                        ncurses::savetty();
                        ncurses::endwin();
                        unix::open_with(pathbuf.as_path(), &args);
                        ncurses::resetty();
                        ncurses::refresh();
                    }
                }
            },
            None => {
                win.destroy();
                ncurses::update_panels();
                ncurses::doupdate();
            }
        }
    }
}

impl command::JoshutoCommand for OpenFileWith {}

impl std::fmt::Display for OpenFileWith {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for OpenFileWith {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if let Some(s) = context.curr_list.as_ref() {
            if let Some(direntry) = s.get_curr_entry() {
                OpenFileWith::open_with(direntry.path.clone(), &context.mimetype_t);
            }
        }
    }
}
