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
use crate::util::worker;

pub fn run(config_t: JoshutoConfig, keymap_t: JoshutoCommandMapping) -> std::io::Result<()> {
    let mut backend: ui::TuiBackend = ui::TuiBackend::new()?;

    let mut context = JoshutoContext::new(config_t);
    let curr_path = std::env::current_dir()?;
    {
        // Initialize an initial tab
        let tab = JoshutoTab::new(curr_path, &context.config_t.sort_option)?;
        context.tab_context_mut().push_tab(tab);

        // trigger a preview of child
        LoadChild::load_child(&mut context)?;

        // render our view
        let view = TuiView::new(&context);
        backend.render(view);
    }

    while !context.exit {
        if !context.worker_is_busy() && !context.worker_is_empty() {
            context.push_msg("started new io_worker task".to_string());
            context.start_next_job();
        }

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            Event::IOWorkerProgress(res) => {
                worker::process_worker_progress(&mut context, res);
            }
            Event::IOWorkerResult(res) => {
                worker::process_finished_worker(&mut context, res);
            }
            Event::Input(key) => {
                if !context.message_queue_ref().is_empty() {
                    context.pop_msg();
                }
                match keymap_t.as_ref().get(&key) {
                    None => {
                        context.push_msg(format!("Unknown keycode: {:?}", key));
                    }
                    Some(CommandKeybind::SimpleKeybind(command)) => {
                        if let Err(e) = command.execute(&mut context, &mut backend) {
                            context.push_msg(e.to_string());
                        }
                    }
                    Some(CommandKeybind::CompositeKeybind(m)) => {
                        let cmd = {
                            let mut menu = TuiCommandMenu::new();
                            menu.get_input(&mut backend, &mut context, &m)
                        };

                        if let Some(command) = cmd {
                            if let Err(e) = command.execute(&mut context, &mut backend) {
                                context.push_msg(e.to_string());
                            }
                        }
                    }
                }
                context.flush_event();
            }
        }
        let view = TuiView::new(&context);
        backend.render(view);
    }

    Ok(())
}
