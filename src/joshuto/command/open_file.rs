extern crate fs_extra;
extern crate ncurses;
extern crate mime_guess;

use std;
use std::env;
use std::fmt;
use std::path;

use joshuto;
use joshuto::command;
use joshuto::input;
use joshuto::config::mimetype;
use joshuto::structs;
use joshuto::ui;
use joshuto::unix;
use joshuto::window;

#[derive(Clone, Debug)]
pub struct OpenFile;

impl OpenFile {
    pub fn new() -> Self { OpenFile }
    pub fn command() -> &'static str { "open_file" }
    pub fn open(paths: &Vec<path::PathBuf>, context: &mut joshuto::JoshutoContext)
    {
        if paths[0].is_file() {
            let file_ext: Option<&str> = match paths[0].extension() {
                Some(s) => s.to_str(),
                None => None,
                };

            let empty_vec: Vec<mimetype::JoshutoMimetypeEntry> = Vec::new();
            let mimetype_options = match file_ext {
                    Some(file_ext) => {
                        match context.mimetype_t.mimetypes.get(file_ext) {
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
                unix::open_with_entry(paths, &mimetype_options[0]);
                ncurses::resetty();
                ncurses::refresh();
            } else {
                ui::wprint_err(&context.views.bot_win, "Don't know how to open file :(");
            }
            ncurses::doupdate();

        } else if paths[0].is_dir() {
            Self::into_directory(&paths[0], context);
        }
    }

    fn into_directory(path: &path::PathBuf, context: &mut joshuto::JoshutoContext)
    {
        let curr_tab = &mut context.tabs[context.tab_index];

        match env::set_current_dir(path) {
            Ok(_) => {},
            Err(e) => {
                ui::wprint_err(&context.views.bot_win, format!("{}: {:?}", e, path).as_str());
                return;
            }
        }

        {
            let dir_list = curr_tab.parent_list.take();
            curr_tab.history.put_back(dir_list);

            let curr_list = curr_tab.curr_list.take();
            curr_tab.parent_list = curr_list;

            let preview_list = curr_tab.preview_list.take();
            curr_tab.curr_list = preview_list;
        }

        /* update curr_path */
        match path.strip_prefix(curr_tab.curr_path.as_path()) {
            Ok(s) => curr_tab.curr_path.push(s),
            Err(e) => {
                ui::wprint_err(&context.views.bot_win, e.to_string().as_str());
                return;
            }
        }

        if let Some(s) = curr_tab.curr_list.as_ref() {
            if s.contents.len() > 0 {
                let dirent: &structs::JoshutoDirEntry = &s.contents[s.index as usize];
                let new_path: path::PathBuf = dirent.path.clone();

                if new_path.is_dir() {
                    curr_tab.preview_list = match curr_tab.history.pop_or_create(
                                new_path.as_path(), &context.config_t.sort_type) {
                        Ok(s) => { Some(s) },
                        Err(e) => {
                            ui::wprint_err(&context.views.right_win,
                                    e.to_string().as_str());
                            None
                        },
                    };
                } else {
                    ncurses::werase(context.views.right_win.win);
                }
            }
        }

        ui::redraw_view(&context.theme_t, &context.views.left_win,
                curr_tab.parent_list.as_ref());
        ui::redraw_view_detailed(&context.theme_t, &context.views.mid_win,
                curr_tab.curr_list.as_ref());
        ui::redraw_view(&context.theme_t, &context.views.right_win,
                curr_tab.preview_list.as_ref());

        ui::redraw_status(&context.theme_t, &context.views,
                curr_tab.curr_list.as_ref(),
                &curr_tab.curr_path,
                &context.username, &context.hostname);

        ncurses::doupdate();
    }
}

impl command::JoshutoCommand for OpenFile {}

impl std::fmt::Display for OpenFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl command::Runnable for OpenFile {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let paths: Option<Vec<path::PathBuf>> = match context.tabs[context.tab_index].curr_list.as_ref() {
                Some(s) => command::collect_selected_paths(s),
                None => None,
            };
        if let Some(paths) = paths {
            Self::open(&paths, context);
        }
    }
}

#[derive(Clone, Debug)]
pub struct OpenFileWith;

impl OpenFileWith {
    pub fn new() -> Self { OpenFileWith }
    pub fn command() -> &'static str { "open_file_with" }

    pub fn open_with(paths: &Vec<path::PathBuf>, mimetype_t: &mimetype::JoshutoMimetype)
    {
        let mut term_rows: i32 = 0;
        let mut term_cols: i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

        let file_ext: Option<&str> = match paths[0].extension() {
            Some(s) => s.to_str(),
            None => None,
            };

        let empty_vec: Vec<mimetype::JoshutoMimetypeEntry> = Vec::new();
        let mimetype_options = match file_ext {
            Some(file_ext) => {
                match mimetype_t.mimetypes.get(file_ext) {
                    Some(s) => s,
                    None => &empty_vec,
                }
            },
            None => &empty_vec,
            };

        let option_size = mimetype_options.len();
        let win = window::JoshutoPanel::new(option_size as i32 + 2, term_cols,
                (term_rows as usize - option_size - 2, 0));
        ncurses::keypad(win.win, true);

        let mut display_vec: Vec<String> = Vec::with_capacity(option_size);
        for (i, val) in mimetype_options.iter().enumerate() {
            display_vec.push(format!("  {}\t{}", i, val));
        }
        display_vec.sort();

        win.move_to_top();
        ui::display_options(&win, &display_vec);
        ncurses::doupdate();

        ncurses::wmove(win.win, option_size as i32 + 1, 0);
        const PROMPT: &str = ":open_with ";
        ncurses::waddstr(win.win, PROMPT);

        let user_input = input::get_str(&win, (option_size as i32 + 1, PROMPT.len() as i32));

        win.destroy();
        ncurses::update_panels();
        ncurses::doupdate();

        if let Some(user_input) = user_input {
            if user_input.len() == 0 {
                return;
            }
            match user_input.parse::<usize>() {
                Ok(s) => {
                    if s < mimetype_options.len() {
                        ncurses::savetty();
                        ncurses::endwin();
                        unix::open_with_entry(&paths, &mimetype_options[s]);
                        ncurses::resetty();
                        ncurses::refresh();
                    }
                }
                Err(_) => {
                    let args: Vec<String> = user_input.split_whitespace().map(|x| String::from(x)).collect();
                    ncurses::savetty();
                    ncurses::endwin();
                    unix::open_with_args(&paths, &args);
                    ncurses::resetty();
                    ncurses::refresh();
                }
            }
        }
    }
}

impl command::JoshutoCommand for OpenFileWith {}

impl std::fmt::Display for OpenFileWith {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl command::Runnable for OpenFileWith {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if let Some(s) = context.tabs[context.tab_index].curr_list.as_ref() {
            if let Some(paths) = command::collect_selected_paths(s) {
                Self::open_with(&paths, &context.mimetype_t);
            }
        }
    }
}
