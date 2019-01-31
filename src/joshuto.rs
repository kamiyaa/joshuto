use std;
use std::collections::HashMap;
use std::time;
use lazy_static::lazy_static;

pub mod config;

mod command;
mod context;
mod history;
mod preview;
mod sort;
mod structs;
mod textfield;
mod ui;
mod unix;
mod window;

use config::JoshutoTheme;
use config::JoshutoMimetype;
use context::JoshutoContext;
use command::CommandKeybind;
use command::JoshutoCommand;

lazy_static! {
    static ref theme_t: JoshutoTheme = JoshutoTheme::get_config();
    static ref mimetype_t: JoshutoMimetype = JoshutoMimetype::get_config();
}

fn recurse_get_keycommand<'a>(keymap: &'a HashMap<i32, CommandKeybind>)
    -> Option<&Box<dyn JoshutoCommand>>
{
    let (term_rows, term_cols) = ui::getmaxyx();
    ncurses::timeout(-1);

    let ch: i32;
    {
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

        ch = ncurses::wgetch(win.win);
    }
    ncurses::doupdate();

    if ch == config::keymap::ESCAPE {
        return None;
    }

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
                something_finished = true;
                break;
            } else {
                let percent = (progress_info.bytes_finished as f64 /
                        progress_info.total_bytes as f64) as f32;
                ui::draw_progress_bar(&context.views.bot_win, percent);
                ncurses::wnoutrefresh(context.views.bot_win.win);
                ncurses::doupdate();
            }
        }
    }
    if something_finished {
        // command::ReloadDirList::reload(context);
        ncurses::doupdate();
    }
}

fn resize_handler(context: &mut JoshutoContext)
{
    ui::redraw_tab_view(&context.views.tab_win, &context);
    {
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab.refresh(&context.views, &context.config_t,
            &context.username, &context.hostname);
    }
    preview::preview_file(context);
    ncurses::doupdate();
}

pub fn run(config_t: config::JoshutoConfig, keymap_t: config::JoshutoKeymap)
{
    ui::init_ncurses();
    ncurses::doupdate();

    let mut context = context::JoshutoContext::new(config_t);
    command::NewTab::new_tab(&mut context);
    ncurses::doupdate();

    while let Some(ch) = ncurses::get_wch() {
        let ch = match ch {
                ncurses::WchResult::Char(s) => s as i32,
                ncurses::WchResult::KeyCode(s) => s,
            };

        if ch == ncurses::KEY_RESIZE {
            context.views.resize_views();
            resize_handler(&mut context);
            continue;
        }

        if context.threads.len() > 0 {
            ncurses::timeout(0);
            process_threads(&mut context);
        } else {
            ncurses::timeout(-1);
        }

        let keycommand: &std::boxed::Box<dyn JoshutoCommand>;

        match keymap_t.keymaps.get(&ch) {
            Some(CommandKeybind::CompositeKeybind(m)) => {
                match recurse_get_keycommand(&m) {
                    Some(s) => {
                        keycommand = s;
                    }
                    None => {
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
