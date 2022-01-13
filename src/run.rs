use crate::commands::numbered_command;
use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::key_command::{AppExecute, CommandKeybind};
use crate::preview::preview_default;
use crate::tab::JoshutoTab;
use crate::ui;
use crate::ui::views;
use crate::ui::views::TuiView;
use crate::util::input;
use crate::util::to_string::ToString;

use termion::event::{Event, Key};
use tui::layout::Rect;

pub fn run(
    backend: &mut ui::TuiBackend,
    context: &mut AppContext,
    keymap_t: AppKeyMapping,
) -> std::io::Result<()> {
    let curr_path = std::env::current_dir()?;
    {
        // Initialize an initial tab
        let tab = JoshutoTab::new(
            curr_path,
            context.ui_context_ref(),
            context.config_ref().display_options_ref(),
        )?;
        context.tab_context_mut().push_tab(tab);

        // trigger a preview of child
        preview_default::load_preview(context, backend);
    }

    while context.quit == QuitType::DoNot {
        // do the ui
        if let Ok(area) = backend.terminal_ref().size() {
            // pre-calculate some ui attributes
            calculate_ui_context(context, area);

            // render the ui
            backend.render(TuiView::new(context));

            // invoke preview hooks, if appropriate
            context.update_external_preview();
        }

        // wait for an event and pop it
        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        // handle the event
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
                    Event::Unsupported(s) => {
                        input::process_unsupported(context, backend, &keymap_t, s);
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

        // update the file system supervisor that watches for changes in the FS
        if context.config_ref().watch_files {
            context.update_watcher();
        }
    } // end of main loop
    Ok(())
}

fn calculate_ui_context(context: &mut AppContext, area: Rect) {
    let area = Rect {
        y: area.top() + 1,
        height: area.height - 2,
        ..area
    };
    let config = context.config_ref();
    let display_options = config.display_options_ref();
    let constraints = views::get_constraints(&context);
    let layout = if display_options.show_borders() {
        views::calculate_layout_with_borders(area, constraints)
    } else {
        views::calculate_layout(area, constraints)
    };
    context.ui_context_mut().layout = layout;
}
