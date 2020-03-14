use std::thread;

use crate::commands::{CommandKeybind, CursorMoveStub, JoshutoRunnable};
use crate::config::{JoshutoCommandMapping, JoshutoConfig};
use crate::context::JoshutoContext;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::ui::widgets::{TuiCommandMenu, TuiView};
use crate::util::event::Event;
use crate::history::DirectoryHistory;
use crate::io::IOWorkerObserver;

pub fn run(config_t: JoshutoConfig, keymap_t: JoshutoCommandMapping) -> std::io::Result<()> {
    let mut backend: ui::TuiBackend = ui::TuiBackend::new()?;

    let mut context = JoshutoContext::new(config_t);
    let curr_path = std::env::current_dir()?;

    {
        // Initialize an initial tab
        let tab = JoshutoTab::new(curr_path, &context.config_t.sort_option)?;
        context.push_tab(tab);

        // move the cursor by 0 just to trigger a preview of child
        CursorMoveStub::new().execute(&mut context, &mut backend);

        // render our view
        let mut view = TuiView::new(&context);
        backend.render(&mut view);
    }

    let mut io_observer = None;
    while !context.exit {
        /* checking if there are workers that need to be run */
        if !context.worker_queue.is_empty() {
            if let None = io_observer.as_ref() {
                let worker = context.worker_queue.pop_front().unwrap();
                io_observer = {
                    let event_tx = context.events.event_tx.clone();
                    let observer = IOWorkerObserver::new(worker, event_tx);
                    Some(observer)
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
                        match io_observer {
                            Some(handle) => {
                                let src = handle.src.clone();
                                let dest = handle.dest.clone();
                                handle.join();
                                context
                                    .message_queue
                                    .push_back("io_worker done".to_string());
                                let options = &context.config_t.sort_option;
                                for tab in context.tabs.iter_mut() {
                                    tab.history.create_or_update(src.as_path(), options);
                                    tab.history.create_or_update(dest.as_path(), options);
                                }
                            }
                            None => {}
                        }
                        io_observer = None;
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
                                let cmd = {
                                    let mut menu = TuiCommandMenu::new();
                                    menu.get_input(&mut backend, &context, &m)
                                };

                                if let Some(command) = cmd {
                                    if let Err(e) = command.execute(&mut context, &mut backend) {
                                        context.message_queue.push_back(e.to_string());
                                    }
                                }
                            }
                        }
                        context.events.flush();
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
    Ok(())
}
