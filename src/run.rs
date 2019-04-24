use std::collections::HashMap;
use std::process;
use std::time;

use crate::commands::{CommandKeybind, FileOperationThread, JoshutoCommand};
use crate::config;
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::preview;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::window::JoshutoPanel;
use crate::window::JoshutoView;

fn recurse_get_keycommand(keymap: &HashMap<i32, CommandKeybind>) -> Option<&Box<JoshutoCommand>> {
    let (term_rows, term_cols) = ui::getmaxyx();
    ncurses::timeout(-1);

    let ch: i32 = {
        let keymap_len = keymap.len();
        let win = JoshutoPanel::new(
            keymap_len as i32 + 1,
            term_cols,
            ((term_rows - keymap_len as i32 - 2) as usize, 0),
        );

        let mut display_vec: Vec<String> = keymap
            .iter()
            .map(|(k, v)| format!("  {}\t{}", *k as u8 as char, v))
            .collect();
        display_vec.sort();

        win.move_to_top();
        ui::display_options(&win, &display_vec);
        ncurses::doupdate();

        ncurses::wgetch(win.win)
    };
    ncurses::doupdate();

    if ch == config::keymap::ESCAPE {
        None
    } else {
        match keymap.get(&ch) {
            Some(CommandKeybind::CompositeKeybind(m)) => recurse_get_keycommand(&m),
            Some(CommandKeybind::SimpleKeybind(s)) => Some(s),
            _ => None,
        }
    }
}

fn join_thread(
    context: &mut JoshutoContext,
    thread: FileOperationThread,
    view: &JoshutoView,
) -> Result<(), std::io::Error> {
    ncurses::werase(view.bot_win.win);
    ncurses::doupdate();

    let (tab_src, tab_dest) = (thread.tab_src, thread.tab_dest);
    match thread.handle.join() {
        Err(e) => {
            ui::wprint_err(&view.bot_win, format!("{:?}", e).as_str());
            view.bot_win.queue_for_refresh();
        }
        Ok(_) => {
            if tab_src < context.tabs.len() {
                let dirty_tab = &mut context.tabs[tab_src];
                dirty_tab.reload_contents(&context.config_t.sort_option)?;
                if tab_src == context.curr_tab_index {
                    dirty_tab.refresh(
                        view,
                        &context.config_t,
                        &context.username,
                        &context.hostname,
                    );
                    preview::preview_file(dirty_tab, view, &context.config_t);
                }
            }
            if tab_dest != tab_src && tab_dest < context.tabs.len() {
                let dirty_tab = &mut context.tabs[tab_dest];
                dirty_tab.reload_contents(&context.config_t.sort_option)?;
                if tab_dest == context.curr_tab_index {
                    dirty_tab.refresh(
                        view,
                        &context.config_t,
                        &context.username,
                        &context.hostname,
                    );
                    preview::preview_file(dirty_tab, view, &context.config_t);
                    ncurses::doupdate();
                }
            }
        }
    }
    Ok(())
}

fn process_threads(context: &mut JoshutoContext, view: &JoshutoView) -> Result<(), std::io::Error> {
    let thread_wait_duration: time::Duration = time::Duration::from_millis(100);
    for i in 0..context.threads.len() {
        match &context.threads[i].recv_timeout(&thread_wait_duration) {
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                let thread = context.threads.swap_remove(i);
                join_thread(context, thread, view)?;
                ncurses::doupdate();
            }
            Ok(progress_info) => {
                ui::draw_progress_bar(
                    &view.bot_win,
                    progress_info.bytes_finished as f32 / progress_info.total_bytes as f32,
                );
            }
            _ => {}
        }
    }
    Ok(())
}

#[inline]
fn resize_handler(context: &mut JoshutoContext, view: &JoshutoView) {
    ui::redraw_tab_view(&view.tab_win, &context);
    {
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab.refresh(
            view,
            &context.config_t,
            &context.username,
            &context.hostname,
        );
        preview::preview_file(curr_tab, view, &context.config_t);
    }
    ncurses::doupdate();
}

fn init_context(context: &mut JoshutoContext, view: &JoshutoView) {
    match std::env::current_dir() {
        Ok(curr_path) => match JoshutoTab::new(curr_path, &context.config_t.sort_option) {
            Ok(tab) => {
                context.tabs.push(tab);
                context.curr_tab_index = context.tabs.len() - 1;
                {
                    let curr_tab = &mut context.tabs[context.curr_tab_index];
                    if let Some(s) = curr_tab.curr_list.as_mut() {
                        if s.need_update() {
                            s.update_contents(&context.config_t.sort_option);
                        }
                    }
                    if let Some(s) = curr_tab.parent_list.as_mut() {
                        if s.need_update() {
                            s.update_contents(&context.config_t.sort_option);
                        }
                    }
                    curr_tab.refresh(
                        view,
                        &context.config_t,
                        &context.username,
                        &context.hostname,
                    );
                }
                ui::redraw_tab_view(&view.tab_win, &context);
                let curr_tab = &mut context.tabs[context.curr_tab_index];
                preview::preview_file(curr_tab, view, &context.config_t);
                ncurses::doupdate();
            }
            Err(e) => {
                ui::end_ncurses();
                eprintln!("{}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            ui::end_ncurses();
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

pub fn run(config_t: config::JoshutoConfig, keymap_t: config::JoshutoKeymap) {
    ui::init_ncurses();

    let mut context = JoshutoContext::new(config_t);
    let mut view = JoshutoView::new(context.config_t.column_ratio);
    init_context(&mut context, &view);

    while !context.exit {
        if !context.threads.is_empty() {
            ncurses::timeout(0);
            match process_threads(&mut context, &view) {
                Ok(()) => {}
                Err(e) => {
                    ui::wprint_err(&view.bot_win, e.to_string().as_str());
                    ncurses::doupdate();
                }
            }
        } else {
            ncurses::timeout(-1);
        }

        if let Some(ch) = ncurses::get_wch() {
            let ch = match ch {
                ncurses::WchResult::Char(s) => s as i32,
                ncurses::WchResult::KeyCode(s) => s,
            };

            if ch == ncurses::KEY_RESIZE {
                view.resize_views();
                resize_handler(&mut context, &view);
                continue;
            }

            let keycommand: &Box<JoshutoCommand>;

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
                    ui::wprint_err(&view.bot_win, &format!("Unknown keycode: {}", ch));
                    ncurses::doupdate();
                    continue;
                }
            }
            match keycommand.execute(&mut context, &view) {
                Ok(()) => {}
                Err(JoshutoError::IO(e)) => {
                    ui::wprint_err(&view.bot_win, e.to_string().as_str());
                    ncurses::doupdate();
                }
            }
        }
    }
    ui::end_ncurses();
}
