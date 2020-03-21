use crate::commands::{CommandKeybind, JoshutoRunnable};
use crate::config::{JoshutoCommandMapping, JoshutoConfig};
use crate::context::JoshutoContext;
use crate::history::DirectoryHistory;
use crate::io::IOWorkerObserver;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::ui::widgets::{TuiCommandMenu, TuiView};
use crate::util::event::Event;
use crate::util::format;
use crate::util::load_child::LoadChild;

pub fn run(config_t: JoshutoConfig, keymap_t: JoshutoCommandMapping) -> std::io::Result<()> {
    let mut backend: ui::TuiBackend = ui::TuiBackend::new()?;

    let mut context = JoshutoContext::new(config_t);
    let curr_path = std::env::current_dir()?;

    {
        // Initialize an initial tab
        let tab = JoshutoTab::new(curr_path, &context.config_t.sort_option)?;
        context.push_tab(tab);

        // trigger a preview of child
        LoadChild::load_child(&mut context)?;

        // render our view
        let mut view = TuiView::new(&context);
        backend.render(&mut view);
    }

    let mut io_observer = None;
    while !context.exit {
        /* checking if there are workers that need to be run */
        if !context.worker_queue.is_empty() {
            if io_observer.is_none() {
                let worker = context.worker_queue.pop_front().unwrap();
                io_observer = {
                    let event_tx = context.events.event_tx.clone();
                    let observer = IOWorkerObserver::new(worker, event_tx);
                    Some(observer)
                };
            }
        }

        let event = match context.events.next() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            Event::IOWorkerProgress(p) => {
                context.worker_msg = Some(format!("bytes copied {}", p));
            }
            Event::IOWorkerResult(res) => {
                match io_observer {
                    Some(handle) => {
                        let src = handle.src.clone();
                        let dest = handle.dest.clone();
                        handle.join();
                        let msg = match res {
                            Ok(s) => {
                                let size_string = format::file_size_to_string(s);
                                format!(
                                    "io_worker completed successfully: {} processed",
                                    size_string)
                            }
                            Err(e) => format!("io_worker was not completed: {}", e.to_string()),
                        };
                        context.message_queue.push_back(msg);
                        let options = &context.config_t.sort_option;
                        for tab in context.tabs.iter_mut() {
                            tab.history.reload(&src, options)?;
                            tab.history.reload(&dest, options)?;
                        }
                        LoadChild::load_child(&mut context)?;
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
                match keymap_t.as_ref().get(&key) {
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
    Ok(())
}
