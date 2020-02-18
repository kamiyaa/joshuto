use std::thread;

use termion::event::Key;

use crate::commands::{CommandKeybind, CursorMoveUp, JoshutoCommand, JoshutoRunnable};
use crate::config::{JoshutoCommandMapping, JoshutoConfig};
use crate::context::JoshutoContext;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::util::event::{Event, Events};
use crate::ui::widgets::TuiCommandMenu;

pub fn run(config_t: JoshutoConfig, keymap_t: JoshutoCommandMapping) {
    let mut backend: ui::TuiBackend = ui::TuiBackend::new().unwrap();

    let mut context = JoshutoContext::new(config_t);
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
                        eprintln!("{}", &format!("bytes copied {}", p));
                    }
                    Event::IOWorkerResult => {
                        match io_handle {
                            Some(handle) => {
                                handle.join();
                                eprintln!("io_worker done");
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
                                eprintln!("{}", e.cause());
                            }
                        }
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            let mut map: &JoshutoCommandMapping = &m;

                            let cmd = {
                                let mut menu = TuiCommandMenu::new();
                                menu.get_input(&mut backend, &context, map)
                            };

                            if let Some(command) = cmd {
                                if let Err(e) = command.execute(&mut context, &mut backend) {
                                    eprintln!("{}", e.cause());
                                }
                            }
                        }
                    },
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
