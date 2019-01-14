extern crate fs_extra;
extern crate ncurses;

use std;
use std::env;
use std::fmt;
use std::path;

use joshuto;
use joshuto::command;
use joshuto::input;
use joshuto::config::mimetype;
use joshuto::ui;
use joshuto::unix;
use joshuto::window;

#[derive(Clone, Debug)]
pub struct OpenFile;

impl OpenFile {
    pub fn new() -> Self { OpenFile }
    pub fn command() -> &'static str { "open_file" }
    pub fn get_options<'a>(path: &path::PathBuf, mimetype_t: &'a mimetype::JoshutoMimetype)
            -> Vec<&'a mimetype::JoshutoMimetypeEntry>
    {
        let mut mimetype_options: Vec<&mimetype::JoshutoMimetypeEntry> = Vec::new();

        match path.extension() {
            Some(file_ext) => {
                if let Some(file_ext) = file_ext.to_str() {
                    match mimetype_t.extensions.get(file_ext) {
                        Some(s) => {
                            for option in s {
                                mimetype_options.push(&option);
                            }
                        }
                        None => {},
                    }
                }
            },
            None => {},
        }
        let detective = mime_detective::MimeDetective::new().unwrap();
        match detective.detect_filepath(path) {
            Ok(mime_type) => {
                match mimetype_t.mimetypes.get(mime_type.type_().as_str()) {
                    Some(s) => {
                        for option in s {
                            mimetype_options.push(&option);
                        }
                    }
                    None => {},
                }
            }
            Err(_) => {},
        }
        mimetype_options
    }

    pub fn open(paths: &Vec<path::PathBuf>, context: &mut joshuto::JoshutoContext)
    {
        if paths[0].is_file() {
            Self::into_file(paths, context);
        } else if paths[0].is_dir() {
            Self::into_directory(&paths[0], context);
            ui::refresh(context);
            ncurses::doupdate();
        } else {
            ui::wprint_err(&context.views.bot_win, "Don't know how to open file :(");
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
            let parent_list = curr_tab.parent_list.take();
            curr_tab.history.put_back(parent_list);

            let curr_list = curr_tab.curr_list.take();
            curr_tab.parent_list = curr_list;
        }

        curr_tab.curr_list = match curr_tab.history.pop_or_create(&path, &context.config_t.sort_type) {
                Ok(s) => Some(s),
                Err(e) => {
                    ui::wprint_err(&context.views.left_win, e.to_string().as_str());
                    None
                },
            };

        /* update curr_path */
        match path.strip_prefix(curr_tab.curr_path.as_path()) {
            Ok(s) => curr_tab.curr_path.push(s),
            Err(e) => {
                ui::wprint_err(&context.views.bot_win, e.to_string().as_str());
                return;
            }
        }
    }

    fn into_file(paths: &Vec<path::PathBuf>, context: &joshuto::JoshutoContext)
    {
        let mimetype_options = Self::get_options(&paths[0], &context.mimetype_t);

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
            if paths.len() > 0 {
                Self::open(&paths, context);
            } else {
                ui::wprint_msg(&context.views.bot_win, "No files selected: 0");
            }
        } else {
            ui::wprint_msg(&context.views.bot_win, "No files selected: None");
        }
        ncurses::doupdate();
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

        let mimetype_options: Vec<&mimetype::JoshutoMimetypeEntry> = OpenFile::get_options(&paths[0], mimetype_t);

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
