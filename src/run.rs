use std::thread;

use termion::event::Key;

use crate::commands::{CommandKeybind, CursorMoveUp, JoshutoCommand, JoshutoRunnable};
use crate::config::{JoshutoCommandMapping, JoshutoConfig};
use crate::context::JoshutoContext;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::util::event::{Event, Events};
use crate::util::menu::OptionMenu;
use crate::window::JoshutoPanel;
use crate::window::JoshutoView;

fn recurse_get_keycommand<'a>(
    events: &Events,
    keymap: &'a JoshutoCommandMapping,
    backend: &'a mut ui::TuiBackend
) -> Option<&'a dyn JoshutoCommand> {
    let event = {
        let mut menu = OptionMenu::new(backend, events);
        let keymap_len = keymap.len();

        // TODO: format keys better, rather than debug
        let mut display_vec: Vec<String> = keymap
            .iter()
            .map(|(k, v)| format!("  {:?}\t{}", k, v))
            .collect();
        display_vec.sort();
        let display_str: Vec<&str> = display_vec
            .iter()
            .map(|v| v.as_str())
            .collect();
        let result = menu.get_option(&display_str);
        eprintln!("{:?}", result);

        let event = events.next();
        event
    };

    let command = match event {
        Ok(Event::Input(input)) => match input {
            Key::Esc => None,
            key @ Key::Char(_) => match keymap.get(&key) {
                Some(CommandKeybind::CompositeKeybind(m)) => recurse_get_keycommand(events, &m, backend),
                Some(CommandKeybind::SimpleKeybind(s)) => Some(s.as_ref()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    };
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
    {
        let tmp = CursorMoveUp::new(0);
        tmp.execute(&mut context, &mut backend);
    }
    backend.render(&context);

    let mut io_handle = None;
    while !context.exit {
        /* checking if there are workers that need to be run */
        if !context.worker_queue.is_empty() {
            if let None = io_handle.as_ref() {
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

        match context.events.next() {
            Ok(event) => {
                match event {
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
                    Event::Input(key) => match keymap_t.get(&key) {
                        None => {
                            eprintln!("Unknown keycode: {:?}", key);
                        }
                        Some(CommandKeybind::SimpleKeybind(command)) => {
                            if let Err(e) = command.execute(&mut context, &mut backend) {
                                ui::wprint_err(&view.bot_win, e.cause());
                            }
                        }
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            let mut cmd = None;
                            let mut map: &JoshutoCommandMapping = &m;

                            loop {
                                eprintln!("run loop");
                                let event2 = {
                                    let mut menu = OptionMenu::new(&mut backend, &context.events);

                                    // TODO: format keys better, rather than debug
                                    let mut display_vec: Vec<String> = map
                                        .iter()
                                        .map(|(k, v)| format!("  {:?}\t{}", k, v))
                                        .collect();
                                    display_vec.sort();
                                    let display_str: Vec<&str> = display_vec
                                        .iter()
                                        .map(|v| v.as_str())
                                        .collect();
                                    let result = menu.get_option(&display_str);
                                    eprintln!("{:?}", result);

                                    result
                                };

                                match event2 {
                                    Some(key) => {
                                        match key {
                                            Key::Esc => {
                                                break;
                                            }
                                            Key::Char(_) => match map.get(&key) {
                                                Some(CommandKeybind::CompositeKeybind(m)) => map = &m,
                                                Some(CommandKeybind::SimpleKeybind(s)) => {
                                                    cmd = Some(s.as_ref());
                                                    break;
                                                }
                                                None => break,
                                            },
                                            _ => {},
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            eprintln!("cmd: {:#?}", cmd);
                            if let Some(command) = cmd {
                                if let Err(e) = command.execute(&mut context, &mut backend) {
                                    ui::wprint_err(&view.bot_win, e.cause());
                                }
                            }
                        }
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
}
