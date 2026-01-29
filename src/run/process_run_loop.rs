use super::process_event;

use crate::commands::quit::QuitAction;
use crate::history::generate_entries_to_root;
use crate::history::DirectoryHistory;
use crate::history::JoshutoHistory;
use crate::preview::preview_default;
use crate::tab::JoshutoTab;
use crate::traits::app_execute::AppExecute;
use crate::traits::ToString;
use crate::types::event::AppEvent;
use crate::types::keybind::CommandKeybind;
use crate::types::keymap::AppKeyMapping;
use crate::types::state::calculate_external_preview;
use crate::types::state::AppState;
use crate::ui;
use crate::ui::views;
use crate::ui::views::TuiView;
use crate::ui::AppBackend;

use uuid::Uuid;

use ratatui::layout::Rect;
use ratatui::termion::event::Event;

pub fn run_loop(
    backend: &mut ui::AppBackend,
    app_state: &mut AppState,
    keymap_t: AppKeyMapping,
) -> std::io::Result<()> {
    let curr_path = std::env::current_dir()?;

    if let Ok(size) = backend.terminal_ref().size() {
        let area = Rect {
            x: 0,
            y: 0,
            width: size.width,
            height: size.height,
        };
        // pre-calculate some ui attributes
        calculate_ui_state(app_state, area);
    }

    {
        let id = Uuid::new_v4();
        // Initialize an initial tab
        let mut new_tab_history = JoshutoHistory::new();
        let tab_display_options = app_state
            .config
            .display_options
            .default_tab_display_option
            .clone();
        let dirlists = generate_entries_to_root(
            curr_path.as_path(),
            &new_tab_history,
            app_state.state.ui_state_ref(),
            &app_state.config.display_options,
            &tab_display_options,
        )?;
        new_tab_history.insert_entries(dirlists);

        let tab = JoshutoTab::new(curr_path, new_tab_history, tab_display_options)?;
        app_state.state.tab_state_mut().insert_tab(id, tab, true);

        // trigger a preview of child
        preview_default::load_previews(app_state, backend);
    }

    while app_state.quit == QuitAction::DoNot {
        // do the ui
        if let Ok(size) = backend.terminal_ref().size() {
            let area = Rect {
                x: 0,
                y: 0,
                width: size.width,
                height: size.height,
            };
            // pre-calculate some ui attributes
            calculate_ui_state(app_state, area);

            // render the ui
            backend.render(TuiView::new(app_state));

            // invoke preview hooks, if appropriate
            {
                let new_preview_area = calculate_external_preview(
                    app_state.state.tab_state_ref(),
                    app_state.state.preview_state_ref(),
                    app_state.state.ui_state_ref(),
                    &app_state.config.preview_options,
                );
                app_state
                    .state
                    .preview_state_mut()
                    .update_external_preview(new_preview_area);
            }
        }

        // wait for an event and pop it
        let event = match app_state.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        // update the file system supervisor that watches for changes in the FS
        if app_state.config.watch_files {
            app_state.state.update_watcher();
        }

        // process user input
        process_input(app_state, backend, &keymap_t, event);
    } // end of main loop
    Ok(())
}

#[inline]
fn process_input(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
    event: AppEvent,
) {
    // handle the event
    match event {
        AppEvent::TerminalEvent(Event::Mouse(event)) => {
            process_event::process_mouse(app_state, backend, keymap_t, event);
            preview_default::load_previews(app_state, backend);
        }
        AppEvent::TerminalEvent(key) => {
            if app_state
                .state
                .message_queue_ref()
                .current_message()
                .is_some()
            {
                app_state.state.message_queue_mut().pop_front();
            }
            match key {
                // in the event where mouse input is not supported
                // but we still want to register scroll
                Event::Unsupported(s) => {
                    process_event::process_unsupported(app_state, backend, keymap_t, s);
                }
                key => match keymap_t.default_view.get(&key) {
                    None => {
                        app_state
                            .state
                            .message_queue_mut()
                            .push_info(format!("Unmapped input: {}", key.to_string()));
                    }
                    Some(CommandKeybind::SimpleKeybind { commands, .. }) => {
                        for command in commands {
                            if let Err(e) = command.execute(app_state, backend, keymap_t) {
                                app_state
                                    .state
                                    .message_queue_mut()
                                    .push_error(e.to_string());
                                break;
                            }
                        }
                    }
                    Some(CommandKeybind::CompositeKeybind(m)) => {
                        let commands =
                            process_event::poll_event_until_simple_keybind(app_state, backend, m);

                        if let Some(commands) = commands {
                            for command in commands {
                                if let Err(e) = command.execute(app_state, backend, keymap_t) {
                                    app_state
                                        .state
                                        .message_queue_mut()
                                        .push_error(e.to_string());
                                    break;
                                }
                            }
                        }
                    }
                },
            }
            preview_default::load_previews(app_state, backend);
            app_state.flush_event();
        }
        event => process_event::process_noninteractive(event, app_state),
    }
}

fn calculate_ui_state(app_state: &mut AppState, area: Rect) {
    let area = Rect {
        y: area.top() + 1,
        height: area.height - 2,
        ..area
    };
    let display_options = &app_state.config.display_options;
    let constraints = views::get_constraints(app_state);
    let layout = if display_options.show_borders {
        views::calculate_layout_with_borders(area, constraints)
    } else {
        views::calculate_layout(area, constraints)
    };
    app_state.state.ui_state_mut().layout = layout;
}
