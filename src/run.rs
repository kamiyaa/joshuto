use std::process;
use std::thread;

use termion::event::Key;

use crate::commands::{CommandKeybind, JoshutoCommand, ReloadDirList};
use crate::config::{JoshutoCommandMapping, JoshutoConfig};
use crate::context::JoshutoContext;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::util::event::{Event, Events};
use crate::window::JoshutoPanel;
use crate::window::JoshutoView;

fn recurse_get_keycommand<'a>(
    events: &Events,
    keymap: &'a JoshutoCommandMapping,
) -> Option<&'a dyn JoshutoCommand> {
    let (term_rows, term_cols) = ui::getmaxyx();

    let event = {
        let keymap_len = keymap.len();
        let win = JoshutoPanel::new(
            keymap_len as i32 + 1,
            term_cols,
            ((term_rows - keymap_len as i32 - 2) as usize, 0),
        );

        let mut display_vec: Vec<String> = keymap
            .iter()
            .map(|(k, v)| format!("  {:?}\t{}", k, v))
            .collect();
        display_vec.sort();

        win.move_to_top();
        ui::display_menu(&win, &display_vec);
        ncurses::doupdate();

        events.next()
    };
    ncurses::doupdate();

    let command = match event {
        Ok(Event::Input(input)) => match input {
            Key::Esc => None,
            key @ Key::Char(_) => match keymap.get(&key) {
                Some(CommandKeybind::CompositeKeybind(m)) => recurse_get_keycommand(events, &m),
                Some(CommandKeybind::SimpleKeybind(s)) => Some(s.as_ref()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    };
    ncurses::doupdate();
    command
}

fn reload_tab(
    index: usize,
    context: &mut JoshutoContext,
    view: &JoshutoView,
) -> std::io::Result<()> {
    ReloadDirList::reload(index, context)?;
    if index == context.curr_tab_index {
        let dirty_tab = &mut context.tabs[index];
        dirty_tab.refresh(view, &context.config_t);
    }
    Ok(())
}

#[inline]
fn resize_handler(context: &mut JoshutoContext, view: &JoshutoView) {
    ui::redraw_tab_view(&view.tab_win, &context);

    let curr_tab = &mut context.tabs[context.curr_tab_index];
    curr_tab.refresh(view, &context.config_t);
    ncurses::doupdate();
}

fn init_context(context: &mut JoshutoContext, view: &JoshutoView) {
    match std::env::current_dir() {
        Ok(curr_path) => match JoshutoTab::new(curr_path, &context.config_t.sort_option) {
            Ok(tab) => {
                context.tabs.push(tab);
                context.curr_tab_index = context.tabs.len() - 1;

                ui::redraw_tab_view(&view.tab_win, &context);
                let curr_tab = &mut context.tabs[context.curr_tab_index];
                curr_tab.refresh(view, &context.config_t);
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

pub fn run(config_t: JoshutoConfig, keymap_t: JoshutoCommandMapping) {
    ui::init_ncurses();

    let mut context = JoshutoContext::new(config_t);
    let mut view = JoshutoView::new(context.config_t.column_ratio);
    init_context(&mut context, &view);
    ncurses::doupdate();

    let mut io_handle = None;
    while !context.exit {
        /* checking if there are workers that need to be run */
        match io_handle.as_ref() {
            None => {
                if !context.worker_queue.is_empty() {
                    let worker = context.worker_queue.pop_front().unwrap();
                    io_handle = {
                        let event_tx = context.events.event_tx.clone();
                        let sync_tx = context.events.sync_tx.clone();
                        let thread = thread::spawn(move || {
                            worker.start();
                            while let Ok(evt) = worker.recv() {
                                event_tx.send(evt);
                                sync_tx.send(());
                            }
                            worker.handle.join();
                            event_tx.send(Event::IOWorkerResult);
                            sync_tx.send(());
                        });
                        Some(thread)
                    };
                }
            }
            _ => {}
        }

        let event = context.events.next();
        if let Ok(event) = event {
            match event {
                Event::Input(key) => {
                    let keycommand = match keymap_t.get(&key) {
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            match recurse_get_keycommand(&context.events, &m) {
                                Some(s) => s,
                                None => {
                                    ui::wprint_err(
                                        &view.bot_win,
                                        &format!("Unknown keycode: {:?}", key),
                                    );
                                    ncurses::doupdate();
                                    continue;
                                }
                            }
                        }
                        Some(CommandKeybind::SimpleKeybind(s)) => s.as_ref(),
                        None => {
                            ui::wprint_err(&view.bot_win, &format!("Unknown keycode: {:?}", key));
                            ncurses::doupdate();
                            continue;
                        }
                    };
                    match keycommand.execute(&mut context, &view) {
                        Err(e) => {
                            ui::wprint_err(&view.bot_win, e.cause());
                        }
                        _ => {}
                    }
                    ncurses::doupdate();
                }
                Event::IOWorkerProgress(p) => {
                    ui::wprint_err(&view.bot_win, &format!("bytes copied {}", p))
                }
                Event::IOWorkerResult => {
                    match io_handle {
                        Some(handle) => {
                            handle.join();
                            ui::wprint_err(&view.bot_win, "io_worker done");
                        }
                        None => {}
                    }
                    io_handle = None;
                }
            }
            ncurses::doupdate();
        }
    }
    ui::end_ncurses();
}
