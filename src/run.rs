use crate::commands::quit::QuitAction;
use crate::config::clean::keymap::AppKeyMapping;
use crate::context::AppContext;
use crate::event::process_event;
use crate::event::AppEvent;
use crate::key_command::{AppExecute, CommandKeybind};
use crate::preview::preview_default;
use crate::tab::JoshutoTab;
use crate::traits::ToString;
use crate::ui;
use crate::ui::views;
use crate::ui::views::TuiView;
use crate::ui::AppBackend;

use uuid::Uuid;

use ratatui::layout::Rect;
use termion::event::Event;

pub fn run_loop(
    backend: &mut ui::AppBackend,
    context: &mut AppContext,
    keymap_t: AppKeyMapping,
) -> std::io::Result<()> {
    let curr_path = std::env::current_dir()?;

    if let Ok(area) = backend.terminal_ref().size() {
        // pre-calculate some ui attributes
        calculate_ui_context(context, area);
    }

    {
        let id = Uuid::new_v4();
        // Initialize an initial tab
        let tab = JoshutoTab::new(
            curr_path,
            context.ui_context_ref(),
            context.config_ref().display_options_ref(),
        )?;
        context.tab_context_mut().insert_tab(id, tab);

        // trigger a preview of child
        preview_default::load_preview(context, backend);
    }

    while context.quit == QuitAction::DoNot {
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

        // update the file system supervisor that watches for changes in the FS
        if context.config_ref().watch_files {
            context.update_watcher();
        }

        // process user input
        process_input(context, backend, &keymap_t, event);
    } // end of main loop
    Ok(())
}

#[inline]
fn process_input(
    context: &mut AppContext,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
    event: AppEvent,
) {
    // handle the event
    match event {
        AppEvent::Termion(Event::Mouse(event)) => {
            process_event::process_mouse(event, context, backend, keymap_t);
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
                    process_event::process_unsupported(context, backend, keymap_t, s);
                }
                key => match keymap_t.default_view.get(&key) {
                    None => {
                        context
                            .message_queue_mut()
                            .push_info(format!("Unmapped input: {}", key.to_string()));
                    }
                    Some(CommandKeybind::SimpleKeybind { commands, .. }) => {
                        for command in commands {
                            if let Err(e) = command.execute(context, backend, keymap_t) {
                                context.message_queue_mut().push_error(e.to_string());
                                break;
                            }
                        }
                    }
                    Some(CommandKeybind::CompositeKeybind(m)) => {
                        let commands =
                            process_event::poll_event_until_simple_keybind(backend, context, m);

                        if let Some(commands) = commands {
                            for command in commands {
                                if let Err(e) = command.execute(context, backend, keymap_t) {
                                    context.message_queue_mut().push_error(e.to_string());
                                    break;
                                }
                            }
                        }
                    }
                },
            }
            preview_default::load_preview(context, backend);
            context.flush_event();
        }
        event => process_event::process_noninteractive(event, context),
    }
}

fn calculate_ui_context(context: &mut AppContext, area: Rect) {
    let area = Rect {
        y: area.top() + 1,
        height: area.height - 2,
        ..area
    };
    let config = context.config_ref();
    let display_options = config.display_options_ref();
    let constraints = views::get_constraints(context);
    let layout = if display_options.show_borders() {
        views::calculate_layout_with_borders(area, constraints)
    } else {
        views::calculate_layout(area, constraints)
    };
    context.ui_context_mut().layout = layout;
}
