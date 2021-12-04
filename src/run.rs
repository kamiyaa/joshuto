use termion::event::{Event, Key};

use crate::commands::numbered_command;
use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::key_command::{AppExecute, Command, CommandKeybind};
use crate::preview::preview_default;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::ui::views::TuiView;
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
        let tab = JoshutoTab::new(curr_path, context.config_ref().display_options_ref())?;
        context.tab_context_mut().push_tab(tab);

        // trigger a preview of child
        preview_default::load_preview(context, backend);
    }

    while context.quit == QuitType::DoNot {
        backend.render(TuiView::new(context));

        if !context.worker_context_ref().is_busy() && !context.worker_context_ref().is_empty() {
            context.worker_context_mut().start_next_job();
        }

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            AppEvent::Termion(Event::Mouse(event)) => {
                input::process_mouse(event, context, backend, &keymap_t);
                preview_default::load_preview(context, backend);
            }
            AppEvent::Termion(key) => {
                if context.message_queue_ref().current_message().is_some() {
                    context.message_queue_mut().pop_front();
                }
                match key {
                    // in the event where mouse input is not supported
                    // but we still want to register scroll
                    Event::Unsupported(s) if s.as_slice() == [27, 79, 65] => {
                        let command = Command::CursorMoveUp(1);
                        if let Err(e) = command.execute(context, backend, &keymap_t) {
                            context.message_queue_mut().push_error(e.to_string());
                        }
                    }
                    Event::Unsupported(s) if s.as_slice() == [27, 79, 66] => {
                        let command = Command::CursorMoveDown(1);
                        if let Err(e) = command.execute(context, backend, &keymap_t) {
                            context.message_queue_mut().push_error(e.to_string());
                        }
                    }
                    Event::Key(Key::Char(c)) if c.is_numeric() && c != '0' => {
                        if let Err(e) =
                            numbered_command::numbered_command(c, context, backend, &keymap_t)
                        {
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
                            if let Err(e) = command.execute(context, backend, &keymap_t) {
                                context.message_queue_mut().push_error(e.to_string());
                            }
                        }
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            let cmd = input::get_input_while_composite(backend, context, m);

                            if let Some(command) = cmd {
                                if let Err(e) = command.execute(context, backend, &keymap_t) {
                                    context.message_queue_mut().push_error(e.to_string());
                                }
                            }
                        }
                    },
                }
                preview_default::load_preview(context, backend);
                context.flush_event();
            }
            event => input::process_noninteractive(event, context),
        }
    }

    Ok(())
}
