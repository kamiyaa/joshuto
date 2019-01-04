#[allow(dead_code)]
extern crate ncurses;

use std;
use std::collections::HashMap;
use std::env;
use std::path;
use std::process;
use std::sync;
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

    pub threads: Vec<(sync::mpsc::Receiver<command::ProgressInfo>, thread::JoinHandle<i32>)>,
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

pub fn resize_handler(context: &mut JoshutoContext)
{
    context.views.redraw_views();
    ncurses::refresh();

    ui::redraw_view(&context.views.left_win, context.parent_list.as_ref());
    ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
    ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

    ui::redraw_status(&context.views, context.curr_list.as_ref(), &context.curr_path,
            &context.config_t.username, &context.config_t.hostname);

    ncurses::doupdate();
}

pub fn run(config_t: config::JoshutoConfig,
    keymap_t: keymap::JoshutoKeymap,
    mimetype_t: mimetype::JoshutoMimetype)
{
    ui::init_ncurses();

    ncurses::doupdate();

    let mut tabs: Vec<JoshutoContext> = Vec::new();
    {
        let context = JoshutoContext::new(&config_t, &mimetype_t);
        tabs.push(context);
    }

    let index: usize = 0;
    loop {
        let ch: i32 = ncurses::getch();

        if ch == ncurses::KEY_RESIZE {
            resize_handler(&mut tabs[index]);
            continue;
        }

        if tabs[index].threads.len() > 0 {
            ncurses::timeout(0);
        } else {
            ncurses::timeout(-1);
        }

        {
            let mut i = 0;

            while i < tabs[index].threads.len() {
                if let Ok(progress_info) = &tabs[index].threads[i].0.recv() {
                    eprintln!("{}/{}", progress_info.bytes_finished, progress_info.total_bytes);
                    if progress_info.bytes_finished == progress_info.total_bytes {
                        let (rx, chandle) = tabs[index].threads.remove(i);
                        ncurses::werase(tabs[index].views.load_bar.win);
                        ncurses::wnoutrefresh(tabs[index].views.load_bar.win);
                        ncurses::doupdate();
                    } else {
                        let percent = (progress_info.bytes_finished as f64 /
                                progress_info.total_bytes as f64) as f32;
                        ui::draw_loading_bar(&tabs[index].views.load_bar, percent);
                        ncurses::wnoutrefresh(tabs[index].views.load_bar.win);
                        ncurses::doupdate();
                    }
                }
                i = i + 1;
            }
        }

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
