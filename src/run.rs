use termion::event::Event;

use crate::commands::{AppExecute, CommandKeybind, KeyCommand};
use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::preview::preview_default;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::ui::views::{TuiCommandMenu, TuiView};
use crate::util::input;
use crate::util::to_string::ToString;

pub fn run(
    backend: &mut ui::TuiBackend,
    context: &mut AppContext,
    keymap_t: AppKeyMapping,
) -> std::io::Result<()> {
    let curr_path = std::env::current_dir()?;
    {
        // Initialize an initial tab
        let tab = JoshutoTab::new(curr_path, &context.config_ref().display_options_ref())?;
        context.tab_context_mut().push_tab(tab);

        // trigger a preview of child
        preview_default::load_preview(context, backend);
    }

    while context.quit == QuitType::DoNot {
        backend.render(TuiView::new(&context));

        if !context.worker_context_ref().is_busy() && !context.worker_context_ref().is_empty() {
            context.worker_context_mut().start_next_job();
        }

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };
        match event {
            AppEvent::Termion(Event::Mouse(event)) => {
                input::process_mouse(event, context, backend);
                preview_default::load_preview(context, backend);
            }
            AppEvent::Termion(key) => {
                if let Some(_) = context.message_queue_ref().current_message() {
                    context.message_queue_mut().pop_front();
                }
                match key {
                    Event::Unsupported(s) if s.as_slice() == [27, 79, 65] => {
                        let command = KeyCommand::CursorMoveUp(1);
                        if let Err(e) = command.execute(context, backend) {
                            context.message_queue_mut().push_error(e.to_string());
                        }
                    }
                    Event::Unsupported(s) if s.as_slice() == [27, 79, 66] => {
                        let command = KeyCommand::CursorMoveDown(1);
                        if let Err(e) = command.execute(context, backend) {
                            context.message_queue_mut().push_error(e.to_string());
                        }
                    }
                    key => match keymap_t.as_ref().get(&key) {
                        None => {
                            context
                                .message_queue_mut()
                                .push_info(format!("Unmapped input: {}", key.to_string()));
                        }
                        Some(CommandKeybind::SimpleKeybind(command)) => {
                            if let Err(e) = command.execute(context, backend) {
                                context.message_queue_mut().push_error(e.to_string());
                            }
                        }
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            let cmd = {
                                let mut menu = TuiCommandMenu::new();
                                menu.get_input(backend, context, &m)
                            };

                            if let Some(command) = cmd {
                                if let Err(e) = command.execute(context, backend) {
                                    context.message_queue_mut().push_error(e.to_string());
                                }
                            }
                        }
                    },
                }
                context.flush_event();
                preview_default::load_preview(context, backend);
            }
            event => input::process_noninteractive(event, context),
        }
    }

    Ok(())
}
