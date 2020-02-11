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

        // TODO: format keys better, rather than debug
        let mut display_vec: Vec<String> = keymap
            .iter()
            .map(|(k, v)| format!("  {:?}\t{}", k, v))
            .collect();
        display_vec.sort();

        win.move_to_top();
        ui::display_menu(&win, &display_vec);
        ncurses::doupdate();

        let event = events.next();
        event
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

pub fn run(config_t: JoshutoConfig, keymap_t: JoshutoCommandMapping) {
    let mut backend: ui::TuiBackend = ui::TuiBackend::new().unwrap();

    let mut context = JoshutoContext::new(config_t);
    let mut view = JoshutoView::new(context.config_t.column_ratio);
    match std::env::current_dir() {
        Ok(curr_path) => match JoshutoTab::new(curr_path, &context.config_t.sort_option) {
            Ok(s) => context.push_tab(s),
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }
    backend.render(&context);

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
                                let _ = event_tx.send(evt);
                                let _ = sync_tx.send(());
                            }
                            worker.handle.join();
                            let _ = event_tx.send(Event::IOWorkerResult);
                            let _ = sync_tx.send(());
                        });
                        Some(thread)
                    };
                }
            }
            _ => {}
        }

        match context.events.next() {
            Ok(event) => {
                match event {
                    Event::Input(key) => match keymap_t.get(&key) {
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            match recurse_get_keycommand(&context.events, &m) {
                                Some(command) => {
                                    if let Err(e) = command.execute(&mut context, &mut backend) {
                                        ui::wprint_err(&view.bot_win, e.cause());
                                    }
                                }
                                None => {
                                    ui::wprint_err(
                                        &view.bot_win,
                                        &format!("Unknown keycode: {:?}", key),
                                    );
                                }
                            }
                        }
                        Some(CommandKeybind::SimpleKeybind(command)) => {
                            if let Err(e) = command.execute(&mut context, &mut backend) {
                                eprintln!("{}", e.cause());
                            }
                        }
                        None => {
                            eprintln!("Unknown keycode: {:?}", key);
                        }
                    },
                    Event::IOWorkerProgress(p) => {
                        ui::wprint_err(&view.bot_win, &format!("bytes copied {}", p));
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
                backend.render(&context);
            }
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        }
    }
    ui::end_ncurses();
}
