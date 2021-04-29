use termion::event::Event;

use crate::commands::{CommandKeybind, JoshutoRunnable, KeyCommand};
use crate::config::JoshutoCommandMapping;
use crate::context::JoshutoContext;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::ui::views::{TuiCommandMenu, TuiView};
use crate::util::event::JoshutoEvent;
use crate::util::input;
use crate::util::load_child::LoadChild;
use crate::util::to_string::ToString;

pub fn run(
    backend: &mut ui::TuiBackend,
    context: &mut JoshutoContext,
    keymap_t: JoshutoCommandMapping,
) -> std::io::Result<()> {
    let curr_path = std::env::current_dir()?;
    {
        // Initialize an initial tab
        let tab = JoshutoTab::new(curr_path, &context.config_ref().display_options_ref())?;
        context.tab_context_mut().push_tab(tab);

        // trigger a preview of child
        LoadChild::load_child(context)?;
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
                input::process_mouse(event, context, backend);
            }
            JoshutoEvent::Termion(key) => {
                if !context.message_queue_ref().is_empty() {
                    context.pop_msg();
                }
                match key {
                    Event::Unsupported(s) if s.as_slice() == [27, 79, 65] => {
                        let command = KeyCommand::CursorMoveUp(1);
                        if let Err(e) = command.execute(context, backend) {
                            context.push_msg(e.to_string());
                        }
                    }
                    Event::Unsupported(s) if s.as_slice() == [27, 79, 66] => {
                        let command = KeyCommand::CursorMoveDown(1);
                        if let Err(e) = command.execute(context, backend) {
                            context.push_msg(e.to_string());
                        }
                    }
                    key => match keymap_t.as_ref().get(&key) {
                        None => {
                            context.push_msg(format!("Unmapped input: {}", key.to_string()));
                        }
                        Some(CommandKeybind::SimpleKeybind(command)) => {
                            if let Err(e) = command.execute(context, backend) {
                                context.push_msg(e.to_string());
                            }
                        }
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            let cmd = {
                                let mut menu = TuiCommandMenu::new();
                                menu.get_input(backend, context, &m)
                            };

                            if let Some(command) = cmd {
                                if let Err(e) = command.execute(context, backend) {
                                    context.push_msg(e.to_string());
                                }
                            }
                        }
                    },
                }
                context.flush_event();
            }
            event => input::process_noninteractive(event, context),
        }
    }

    Ok(())
}
