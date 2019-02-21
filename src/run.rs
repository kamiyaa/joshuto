use std::collections::HashMap;
use std::time;

use crate::commands;
use crate::commands::{CommandKeybind, JoshutoCommand};
use crate::config;
use crate::context::JoshutoContext;
use crate::preview;
use crate::ui;
use crate::window::JoshutoPanel;

fn recurse_get_keycommand(
    keymap: &HashMap<i32, CommandKeybind>,
) -> Option<&Box<dyn JoshutoCommand>> {
    let (term_rows, term_cols) = ui::getmaxyx();
    ncurses::timeout(-1);

    let ch: i32;
    {
        let keymap_len = keymap.len();
        let win = JoshutoPanel::new(
            keymap_len as i32 + 1,
            term_cols,
            ((term_rows - keymap_len as i32 - 2) as usize, 0),
        );

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
        Some(CommandKeybind::CompositeKeybind(m)) => recurse_get_keycommand(&m),
        Some(CommandKeybind::SimpleKeybind(s)) => Some(s),
        _ => None,
    }
}

fn process_threads(context: &mut JoshutoContext) {
    let thread_wait_duration: time::Duration = time::Duration::from_millis(100);

    let mut i: usize = 0;
    while i < context.threads.len() {
        match &context.threads[i].recv_timeout(&thread_wait_duration) {
            Ok(progress_info) => {
                if progress_info.bytes_finished == progress_info.total_bytes {
                    ncurses::werase(context.views.bot_win.win);
                    let thread = context.threads.swap_remove(i);
                    thread.handle.join().unwrap();
                    let (tab_src, tab_dest) = (thread.tab_src, thread.tab_dest);
                    if tab_src < context.tabs.len() {
                        context.tabs[tab_src].reload_contents(&context.config_t.sort_type);
                        if tab_src == context.curr_tab_index {
                            context.tabs[tab_src].refresh(
                                &context.views,
                                &context.config_t,
                                &context.username,
                                &context.hostname,
                            );
                        }
                    }
                    if tab_dest != tab_src && tab_dest < context.tabs.len() {
                        context.tabs[tab_dest].reload_contents(&context.config_t.sort_type);
                        if tab_dest == context.curr_tab_index {
                            context.tabs[tab_dest].refresh(
                                &context.views,
                                &context.config_t,
                                &context.username,
                                &context.hostname,
                            );
                        }
                    }
                } else {
                    let percent = (progress_info.bytes_finished as f64
                        / progress_info.total_bytes as f64)
                        as f32;
                    ui::draw_progress_bar(&context.views.bot_win, percent);
                    ncurses::wnoutrefresh(context.views.bot_win.win);
                    i += 1;
                }
                ncurses::doupdate();
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                ncurses::werase(context.views.bot_win.win);
                let thread = context.threads.swap_remove(i);
                let (tab_src, tab_dest) = (thread.tab_src, thread.tab_dest);
                thread.handle.join().unwrap();
                if tab_src < context.tabs.len() {
                    context.tabs[tab_src].reload_contents(&context.config_t.sort_type);
                    if tab_src == context.curr_tab_index {
                        context.tabs[tab_src].refresh(
                            &context.views,
                            &context.config_t,
                            &context.username,
                            &context.hostname,
                        );
                    }
                }
                if tab_dest != tab_src && tab_dest < context.tabs.len() {
                    context.tabs[tab_dest].reload_contents(&context.config_t.sort_type);
                    if tab_dest == context.curr_tab_index {
                        context.tabs[tab_dest].refresh(
                            &context.views,
                            &context.config_t,
                            &context.username,
                            &context.hostname,
                        );
                    }
                }
                ncurses::doupdate();
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                i += 1;
            }
        }
    }
}

fn resize_handler(context: &mut JoshutoContext) {
    ui::redraw_tab_view(&context.views.tab_win, &context);
    {
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab.refresh(
            &context.views,
            &context.config_t,
            &context.username,
            &context.hostname,
        );
        preview::preview_file(curr_tab, &context.views, &context.config_t);
    }
    ncurses::doupdate();
}

pub fn run(config_t: config::JoshutoConfig, keymap_t: config::JoshutoKeymap) {
    ui::init_ncurses();

    let mut context = JoshutoContext::new(config_t);
    commands::NewTab::new_tab(&mut context);
    preview::preview_file(&mut context.tabs[context.curr_tab_index], &context.views, &context.config_t);
    ncurses::doupdate();

    while !context.exit {
        if !context.threads.is_empty() {
            ncurses::timeout(0);
            process_threads(&mut context);
        } else {
            ncurses::timeout(-1);
        }

        if let Some(ch) = ncurses::get_wch() {
            let ch = match ch {
                ncurses::WchResult::Char(s) => s as i32,
                ncurses::WchResult::KeyCode(s) => s,
            };

            if ch == ncurses::KEY_RESIZE {
                context.views.resize_views();
                resize_handler(&mut context);
                continue;
            }

            let keycommand: &Box<dyn JoshutoCommand>;

            match keymap_t.keymaps.get(&ch) {
                Some(CommandKeybind::CompositeKeybind(m)) => match recurse_get_keycommand(&m) {
                    Some(s) => {
                        keycommand = s;
                    }
                    None => continue,
                },
                Some(CommandKeybind::SimpleKeybind(s)) => {
                    keycommand = s;
                }
                None => {
                    ui::wprint_err(&context.views.bot_win, &format!("Unknown keycode: {}", ch));
                    ncurses::doupdate();
                    continue;
                }
            }
            keycommand.execute(&mut context);
        }
    }
    ui::end_ncurses();
}
