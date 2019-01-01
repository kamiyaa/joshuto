#[allow(dead_code)]
extern crate ncurses;

use std;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path;
use std::process;
use std::thread;

pub mod config;
pub mod keymap;
pub mod mimetype;
mod command;
mod history;
// mod navigation;
mod sort;
mod structs;
mod ui;
mod unix;
mod window;

mod keymapll;

use self::command::CommandKeybind;
use self::command::JoshutoCommand;
use self::keymapll::Keycode;

pub struct JoshutoContext<'a> {
    pub curr_path: path::PathBuf,
    pub history: history::DirHistory,

    pub threads: Vec<thread::JoinHandle<i32>>,
    pub views: window::JoshutoView,
    pub curr_list: Option<structs::JoshutoDirList>,
    pub parent_list: Option<structs::JoshutoDirList>,
    pub preview_list: Option<structs::JoshutoDirList>,

    pub config_t: config::JoshutoConfig,
    pub mimetype_t: &'a mimetype::JoshutoMimetype,
}

impl<'a> JoshutoContext<'a> {

    pub fn new(config_t: &config::JoshutoConfig,
        mimetype_t: &'a mimetype::JoshutoMimetype) -> Self
    {
        let curr_path : path::PathBuf = match env::current_dir() {
            Ok(path) => { path },
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            },
        };

        /* keep track of where we are in directories */
        let mut history = history::DirHistory::new();
        history.populate_to_root(&curr_path, &config_t.sort_type);

        let joshuto_view: window::JoshutoView =
            window::JoshutoView::new(config_t.column_ratio);

        /* load up directories */
        let curr_view: Option<structs::JoshutoDirList> =
            match history.pop_or_create(&curr_path, &config_t.sort_type) {
                Ok(s) => { Some(s) },
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                },
            };

        let parent_view: Option<structs::JoshutoDirList> =
            match curr_path.parent() {
                Some(parent) => {
                    match history.pop_or_create(&parent, &config_t.sort_type) {
                        Ok(s) => { Some(s) },
                        Err(e) => {
                            eprintln!("{}", e);
                            process::exit(1);
                        },
                    }
                },
                None => { None },
            };

        let preview_view: Option<structs::JoshutoDirList>;
        if let Some(s) = curr_view.as_ref() {
            match s.get_curr_entry() {
                Some(dirent) => {
                    let preview_path = dirent.entry.path();
                    if preview_path.is_dir() {
                        preview_view = match history.pop_or_create(&preview_path, &config_t.sort_type) {
                            Ok(s) => { Some(s) },
                            Err(e) => {
                                eprintln!("{}", e);
                                None
                            },
                        };
                    } else {
                        preview_view = None;
                    }
                },
                None => {
                    preview_view = None;
                }
            }
        } else {
            preview_view = None
        }

        ui::redraw_status(&joshuto_view, curr_view.as_ref(), &curr_path,
                &config_t.username, &config_t.hostname);

        ui::redraw_view(&joshuto_view.left_win, parent_view.as_ref());
        ui::redraw_view(&joshuto_view.mid_win, curr_view.as_ref());
        ui::redraw_view(&joshuto_view.right_win, preview_view.as_ref());

        ncurses::doupdate();

        JoshutoContext {
            curr_path,
            history,
            threads: Vec::new(),
            views: joshuto_view,
            curr_list: curr_view,
            parent_list: parent_view,
            preview_list: preview_view,

            config_t: config_t.clone(),
            mimetype_t,
        }
    }

    pub fn reload_dirlists(&mut self)
    {
        let mut gone = false;
        if let Some(s) = self.curr_list.as_mut() {
            if !s.path.exists() {
                gone = true;
            } else if s.need_update() {
                s.update(&self.config_t.sort_type);
            }
        }
        if gone {
            self.curr_list = None;
        }

        let mut gone = false;
        if let Some(s) = self.parent_list.as_mut() {
            if !s.path.exists() {
                gone = true;
            } else if s.need_update() {
                s.update(&self.config_t.sort_type);
            }
        }
        if gone {
            self.parent_list = None;
        }

        let mut gone = false;
        if let Some(s) = self.preview_list.as_mut() {
            if !s.path.exists() {
                gone = true;
            } else if s.need_update() {
                s.update(&self.config_t.sort_type);
            }
        }
        if gone {
            self.preview_list = None;
        }
    }
}

fn recurse_get_keycommand<'a>(keymap: &'a HashMap<i32, CommandKeybind>)
    -> Option<&Box<dyn command::JoshutoCommand>>
{
    let mut term_rows: i32 = 0;
    let mut term_cols: i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

    let keymap_len = keymap.len();

    let mut win = window::JoshutoPanel::new(keymap_len as i32 + 1, term_cols,
            ((term_rows - keymap_len as i32 - 2) as usize, 0));

    let mut display_vec: Vec<String> = Vec::with_capacity(keymap_len);
    for (key, val) in keymap {
        display_vec.push(format!("  {}\t{}", *key as u8 as char, val));
    }
    display_vec.sort();

    win.move_to_top();
    ui::display_options(&win, &display_vec);
    ncurses::doupdate();

    let ch: i32 = ncurses::getch();

    win.destroy();
    ncurses::update_panels();
    ncurses::doupdate();

    if ch == Keycode::ESCAPE as i32 {
        None
    } else {
        match keymap.get(&ch) {
            Some(CommandKeybind::CompositeKeybind(m)) => {
                recurse_get_keycommand(&m)
            },
            Some(CommandKeybind::SimpleKeybind(s)) => {
                Some(s)
            },
            _ => {
                None
            }
        }
    }
}

fn open_with(mimetypes: &HashMap<String, Vec<Vec<String>>>,
        direntry: &fs::DirEntry)
{
    let mut term_rows: i32 = 0;
    let mut term_cols: i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

    let pathbuf = direntry.path();
    let mimetype = unix::get_mime_type(pathbuf.as_path());

    let mut empty_vec: Vec<Vec<String>> = Vec::new();
    let mimetype_options: &Vec<Vec<String>>;
    match mimetypes.get(&mimetype) {
        Some(s) => {
            mimetype_options = s;
        },
        None => {
            mimetype_options = &empty_vec;
        },
    }

    let option_size = mimetype_options.len();
    let mut win = window::JoshutoPanel::new(option_size as i32 + 2, term_cols,
            (term_rows as usize - option_size - 2, 0));

    let mut display_vec: Vec<String> = Vec::with_capacity(option_size);
    for (i, val) in mimetype_options.iter().enumerate() {
        display_vec.push(format!("  {}\t{}", i, val.join(" ")));
    }
    display_vec.sort();

    win.move_to_top();
    ui::display_options(&win, &display_vec);
    // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
    ncurses::doupdate();

    ncurses::wmove(win.win, option_size as i32 + 1, 0);
    ncurses::wprintw(win.win, ":open_with ");

    let mut cur_ind = ":open_with ".len();

    let mut user_input: String = String::new();
    loop {
        ncurses::wprintw(win.win, "_");
        ncurses::wmove(win.win, option_size as i32 + 1, cur_ind as i32);
        let ch: i32 = ncurses::wgetch(win.win);
        if ch == Keycode::ESCAPE as i32 {
            win.destroy();
            ncurses::update_panels();
            ncurses::doupdate();
            return;
        }
        if ch == Keycode::ENTER as i32 {
            break;
        }
        if ch == Keycode::BACKSPACE as i32 || ch == 127 {
            match user_input.pop() {
                Some(_) => {
                    cur_ind = cur_ind - 1;
                    ncurses::mvwdelch(win.win, option_size as i32 + 1, cur_ind as i32);
                },
                None => {},
            }
//            ncurses::wmove(win.win, option_size as i32 + 1, cur_ind as i32);
            continue;
        }
        user_input.push(ch as u8 as char);
        cur_ind = cur_ind + 1;

        ncurses::wprintw(win.win, (ch as u8 as char).to_string().as_str());
    }

    win.destroy();
    // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
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
}

fn update_views(joshuto_view : &window::JoshutoView,
        parent_view: Option<&mut structs::JoshutoDirList>,
        curr_view: Option<&mut structs::JoshutoDirList>,
        preview_view: Option<&mut structs::JoshutoDirList>,
        config_t: &config::JoshutoConfig,
        )
{
    if let Some(s) = parent_view {
        if s.update_needed || s.need_update() {
            s.update(&config_t.sort_type);
            s.display_contents(&joshuto_view.left_win);
            ncurses::wnoutrefresh(joshuto_view.left_win.win);
        }
    }

    if let Some(s) = curr_view {
        if s.update_needed || s.need_update() {
            s.update(&config_t.sort_type);
            s.display_contents(&joshuto_view.mid_win);
            ncurses::wnoutrefresh(joshuto_view.mid_win.win);
        }
    }

    if let Some(s) = preview_view {
        if s.update_needed || s.need_update() {
            s.update(&config_t.sort_type);
            s.display_contents(&joshuto_view.right_win);
            ncurses::wnoutrefresh(joshuto_view.right_win.win);
        }
    }

    ncurses::doupdate();
}

pub fn run(mut config_t: config::JoshutoConfig,
    keymap_t: keymap::JoshutoKeymap,
    mimetype_t: mimetype::JoshutoMimetype)
{
    ui::init_ncurses();

    ncurses::doupdate();

    let mut tabs: Vec<JoshutoContext> = Vec::new();
    let mut context = JoshutoContext::new(&config_t, &mimetype_t);
    let mut index: usize = 0;
    tabs.push(context);

    loop {
        let ch: i32 = ncurses::getch();

        let keycommand: &std::boxed::Box<dyn JoshutoCommand>;

        match keymap_t.keymaps.get(&ch) {
            Some(CommandKeybind::CompositeKeybind(m)) => {
                match recurse_get_keycommand(&m) {
                    Some(s) => {
                        ncurses::update_panels();
                        ncurses::doupdate();
                        keycommand = s;
                    }
                    None => {
                        ncurses::update_panels();
                        ncurses::doupdate();
                        continue
                    },
                }

            },
            Some(CommandKeybind::SimpleKeybind(s)) => {
                keycommand = s;
            },
            None => {
                continue;
            }
        }

//        ncurses::printw(format!("{}", *keycommand).as_str());

        keycommand.execute(&mut tabs[index]);
    }
}
