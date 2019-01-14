extern crate ncurses;

use std;
use std::collections::HashMap;
use std::path;
use std::process;
use std::sync;
use std::thread;
use std::time;

pub mod config;

mod command;
mod history;
mod input;
mod preview;
mod sort;
mod structs;
mod ui;
mod unix;
mod window;

use self::command::CommandKeybind;
use self::command::JoshutoCommand;

pub struct JoshutoTab {
    pub history: history::DirHistory,
    pub curr_path: path::PathBuf,
    pub parent_list: Option<structs::JoshutoDirList>,
    pub curr_list: Option<structs::JoshutoDirList>,
}

impl JoshutoTab {
    pub fn new(curr_path: path::PathBuf, sort_type: &sort::SortType) -> Self
    {
        /* keep track of where we are in directories */
        let mut history = history::DirHistory::new();
        history.populate_to_root(&curr_path, sort_type);

        /* load up directories */
        let curr_list: Option<structs::JoshutoDirList> =
            match history.pop_or_create(&curr_path, sort_type) {
                Ok(s) => { Some(s) },
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                },
            };

        let parent_list: Option<structs::JoshutoDirList> =
            match curr_path.parent() {
                Some(parent) => {
                    match history.pop_or_create(&parent, sort_type) {
                        Ok(s) => { Some(s) },
                        Err(e) => {
                            eprintln!("{}", e);
                            process::exit(1);
                        },
                    }
                },
                None => { None },
            };

        JoshutoTab {
                curr_path,
                history,
                curr_list,
                parent_list,
            }
    }
}

pub struct JoshutoContext<'a> {
    pub username: String,
    pub hostname: String,
    pub threads: Vec<(sync::mpsc::Receiver<command::ProgressInfo>, thread::JoinHandle<i32>)>,
    pub views: window::JoshutoView,
    pub tab_index: usize,
    pub tabs: Vec<JoshutoTab>,

    pub config_t: &'a mut config::JoshutoConfig,
    pub mimetype_t: &'a config::JoshutoMimetype,
    pub theme_t: &'a config::JoshutoTheme,
}

impl<'a> JoshutoContext<'a> {
    pub fn new(config_t: &'a mut config::JoshutoConfig,
        mimetype_t: &'a config::JoshutoMimetype,
        theme_t: &'a config::JoshutoTheme) -> Self
    {
        let username: String = whoami::username();
        let hostname: String = whoami::hostname();

        let views: window::JoshutoView =
            window::JoshutoView::new(config_t.column_ratio);

        JoshutoContext {
            username,
            hostname,
            threads: Vec::new(),
            views,
            tab_index: 0,
            tabs: Vec::new(),
            config_t,
            mimetype_t,
            theme_t
        }
    }

    pub fn reload_dirlists(&mut self)
    {
        if self.tab_index >= self.tabs.len() {
            return;
        }

        let mut gone = false;
        if let Some(s) = self.tabs[self.tab_index].curr_list.as_mut() {
            if !s.path.exists() {
                gone = true;
            } else if s.need_update() {
                s.update(&self.config_t.sort_type);
            }
        }
        if gone {
            self.tabs[self.tab_index].curr_list = None;
        }

        let mut gone = false;
        if let Some(s) = self.tabs[self.tab_index].parent_list.as_mut() {
            if !s.path.exists() {
                gone = true;
            } else if s.need_update() {
                s.update(&self.config_t.sort_type);
            }
        }
        if gone {
            self.tabs[self.tab_index].parent_list = None;
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

    let win = window::JoshutoPanel::new(keymap_len as i32 + 1, term_cols,
            ((term_rows - keymap_len as i32 - 2) as usize, 0));

    let mut display_vec: Vec<String> = Vec::with_capacity(keymap_len);
    for (key, val) in keymap {
        display_vec.push(format!("  {}\t{}", *key as u8 as char, val));
    }
    display_vec.sort();

    win.move_to_top();
    ui::display_options(&win, &display_vec);
    ncurses::doupdate();
    ncurses::timeout(-1);

    let ch: i32 = ncurses::getch();

    win.destroy();
    ncurses::update_panels();
    ncurses::doupdate();

    if ch == config::keymap::ESCAPE {
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

fn process_threads(context: &mut JoshutoContext)
{
    let wait_duration: time::Duration = time::Duration::from_millis(100);
    let mut something_finished = false;
    for i in 0..context.threads.len() {
        if let Ok(progress_info) = &context.threads[i].0.recv_timeout(wait_duration) {

            if progress_info.bytes_finished == progress_info.total_bytes {
                let (_, chandle) = context.threads.remove(i);
                chandle.join().unwrap();
                ncurses::werase(context.views.bot_win.win);
                let curr_list = context.tabs[context.tab_index].curr_list.as_ref();
                let curr_path = &context.tabs[context.tab_index].curr_path;
                ui::redraw_status(&context.theme_t, &context.views, curr_list, curr_path,
                        &context.username, &context.hostname);
                ncurses::doupdate();
                something_finished = true;
                break;
            } else {
                let percent = (progress_info.bytes_finished as f64 /
                        progress_info.total_bytes as f64) as f32;
                ui::draw_loading_bar(&context.theme_t, &context.views.bot_win, percent);
                ncurses::wnoutrefresh(context.views.bot_win.win);
                ncurses::doupdate();
            }
        }
    }

    if something_finished {
        command::ReloadDirList::reload(context);
    }
}

fn resize_handler(context: &mut JoshutoContext)
{
    context.views.redraw_views();
    ncurses::refresh();
    ui::refresh(context);

    ui::redraw_tab_view(&context.views.tab_win, &context);
    ncurses::doupdate();
}

pub fn run(mut config_t: config::JoshutoConfig,
    keymap_t: config::JoshutoKeymap,
    mimetype_t: config::JoshutoMimetype,
    theme_t: config::JoshutoTheme)
{
    ui::init_ncurses(&theme_t);
    ncurses::doupdate();

    let mut context = JoshutoContext::new(&mut config_t, &mimetype_t, &theme_t);
    command::NewTab::new_tab(&mut context);
    preview::preview_file(&mut context);
    ui::refresh(&mut context);
    ncurses::doupdate();

    loop {
        let ch: i32 = ncurses::getch();

        if ch == ncurses::KEY_RESIZE {
            resize_handler(&mut context);
            continue;
        }

        if context.threads.len() > 0 {
            ncurses::timeout(0);
        } else {
            ncurses::timeout(-1);
        }

        process_threads(&mut context);

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
        keycommand.execute(&mut context);
    }
}
