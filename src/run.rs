use termion::event::Event;

use crate::commands::{CommandKeybind, JoshutoRunnable};
use crate::config::{JoshutoCommandMapping, JoshutoConfig};
use crate::context::JoshutoContext;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::ui::views::TuiView;
use crate::ui::widgets::TuiCommandMenu;
use crate::util::event::JoshutoEvent;
use crate::util::input_process;
use crate::util::load_child::LoadChild;

pub fn run(config_t: JoshutoConfig, keymap_t: JoshutoCommandMapping) -> std::io::Result<()> {
    let mut backend: ui::TuiBackend = ui::TuiBackend::new()?;

    let mut context = JoshutoContext::new(config_t);
    let curr_path = std::env::current_dir()?;
    {
        // Initialize an initial tab
        let tab = JoshutoTab::new(curr_path, &context.config_ref().sort_option)?;
        context.tab_context_mut().push_tab(tab);

        // trigger a preview of child
        LoadChild::load_child(&mut context)?;
    }

    while !context.exit {
        backend.render(TuiView::new(&context));

        if !context.worker_is_busy() && !context.worker_is_empty() {
            context.start_next_job();
        }

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };
        match event {
            JoshutoEvent::Termion(Event::Mouse(event)) => {
                input_process::process_mouse(event, &mut context, &mut backend);
            }
            JoshutoEvent::Termion(key) => {
                if !context.message_queue_ref().is_empty() {
                    context.pop_msg();
                }
                match keymap_t.as_ref().get(&key) {
                    None => {
                        context.push_msg(format!("Unmapped input: {:?}", key));
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
            event => input_process::process_noninteractive(event, &mut context),
        }
    }

    Ok(())
}
