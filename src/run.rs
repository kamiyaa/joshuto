use crate::commands::CommandKeybind;
use crate::config::{JoshutoCommandMapping, JoshutoConfig};
use crate::context::JoshutoContext;
use crate::history::DirectoryHistory;
use crate::io::FileOp;
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
        let view = TuiView::new(&context);
        backend.render(view);
    }

    while !context.exit {
        if !context.worker.is_empty() && !context.worker.is_busy() {
            let tx = context.events.event_tx.clone();
            context.worker.run_next_job(tx);
        }

        let event = match context.events.next() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            Event::IOWorkerProgress((FileOp::Cut, p)) => {
                context.push_msg(format!("{} moved", format::file_size_to_string(p)));
            }
            Event::IOWorkerProgress((FileOp::Copy, p)) => {
                context.push_msg(format!("{} copied", format::file_size_to_string(p)));
            }
            Event::IOWorkerResult((file_op, Ok(p))) => {
                let observer = context.worker.observer.take().unwrap();
                let options = &context.config_t.sort_option;
                for tab in context.tabs.iter_mut() {
                    tab.history.reload(&observer.src, options)?;
                    tab.history.reload(&observer.dest, options)?;
                }
                let msg = match file_op {
                    FileOp::Copy => format!(
                        "copied {} to {:?}",
                        format::file_size_to_string(p),
                        observer.dest
                    ),
                    FileOp::Cut => format!(
                        "moved {} to {:?}",
                        format::file_size_to_string(p),
                        observer.dest
                    ),
                };
                context.push_msg(msg);
                observer.join();
            }
            Event::IOWorkerResult((_, Err(e))) => {
                let observer = context.worker.observer.take().unwrap();
                let options = &context.config_t.sort_option;
                for tab in context.tabs.iter_mut() {
                    tab.history.reload(&observer.src, options)?;
                    tab.history.reload(&observer.dest, options)?;
                }
                let msg = format!("{}", e);
                context.push_msg(msg);
                observer.join();
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
        let view = TuiView::new(&context);
        backend.render(view);
    }

    Ok(())
}
