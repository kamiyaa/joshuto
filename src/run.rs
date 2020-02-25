use std::thread;

use crate::commands::{CommandKeybind, CursorMoveUp, JoshutoRunnable};
use crate::config::{JoshutoCommandMapping, JoshutoConfig};
use crate::context::{JoshutoContext, MESSAGE_VISIBLE_DURATION};
use crate::tab::JoshutoTab;
use crate::ui;
use crate::ui::widgets::{TuiCommandMenu, TuiView};
use crate::util::event::Event;

pub fn run(config_t: JoshutoConfig, keymap_t: JoshutoCommandMapping) -> std::io::Result<()> {
    let mut backend: ui::TuiBackend = ui::TuiBackend::new()?;

    let mut context = JoshutoContext::new(config_t);
    let curr_path = std::env::current_dir()?;

    {
        // Initialize an initial tab
        let tab = JoshutoTab::new(curr_path, &context.config_t.sort_option)?;
        context.push_tab(tab);

        // move the cursor by 0 just to trigger a preview of child
        let tmp = CursorMoveUp::new(0);
        tmp.execute(&mut context, &mut backend);

        // render our view
        let mut view = TuiView::new(&context);
        backend.render(&mut view);
    }

    let mut io_handle = None;
    while !context.exit {
        /* checking if there are workers that need to be run */
        if !context.worker_queue.is_empty() {
            if let None = io_handle.as_ref() {
                let worker = context.worker_queue.pop_front().unwrap();
                io_handle = {
                    let event_tx = context.events.event_tx.clone();
                    let thread = thread::spawn(move || {
                        worker.start();
                        while let Ok(evt) = worker.recv() {
                            let _ = event_tx.send(evt);
                        }
                        worker.handle.join();
                        let _ = event_tx.send(Event::IOWorkerResult);
                    });
                    Some(thread)
                };
            }
        }

        match context.events.next() {
            Ok(event) => {
                match event {
                    Event::IOWorkerProgress(p) => {
                        context.worker_msg = Some(format!("bytes copied {}", p));
                    }
                    Event::IOWorkerResult => {
                        match io_handle {
                            Some(handle) => {
                                handle.join();
                                context
                                    .message_queue
                                    .push_back("io_worker done".to_string());
                            }
                            None => {}
                        }
                        io_handle = None;
                        context.worker_msg = None;
                    }
                    Event::Input(key) => {
                        /* Message handling */
                        if !context.message_queue.is_empty() {
                            let _ = context.message_queue.pop_front();
                        }
                        match keymap_t.get(&key) {
                            None => {
                                context
                                    .message_queue
                                    .push_back(format!("Unknown keycode: {:?}", key));
                            }
                            Some(CommandKeybind::SimpleKeybind(command)) => {
                                if let Err(e) = command.execute(&mut context, &mut backend) {
                                    context.message_queue.push_back(e.to_string());
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
                                        context.message_queue.push_back(e.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
                let mut view = TuiView::new(&context);
                backend.render(&mut view);
            }
            Err(e) => {
                context.message_queue.push_back(e.to_string());
                break;
            }
        }
    }
    eprintln!("{:#?}", context.message_queue);
    Ok(())
}
