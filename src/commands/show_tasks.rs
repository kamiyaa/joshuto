use crate::error::AppResult;
use crate::run::process_event;
use crate::traits::ToString;
use crate::types::command::Command;
use crate::types::event::AppEvent;
use crate::types::keybind::CommandKeybind;
use crate::types::keymap::AppKeyMapping;
use crate::types::state::remove_external_preview;
use crate::types::state::AppState;
use crate::ui::views::TuiWorkerView;
use crate::ui::AppBackend;

pub fn show_tasks(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
) -> AppResult {
    app_state.flush_event();
    remove_external_preview(app_state);

    let mut exit = false;

    while !exit {
        backend.render(TuiWorkerView::new(app_state));

        if let Ok(event) = app_state.poll_event() {
            match event {
                AppEvent::TerminalEvent(key) => {
                    match keymap_t.task_view.get(&key) {
                        None => {
                            app_state
                                .state
                                .message_queue_mut()
                                .push_info(format!("Unmapped input: {}", key.to_string()));
                        }
                        Some(CommandKeybind::SimpleKeybind { commands, .. }) => {
                            for command in commands {
                                if let Command::ShowTasks = command {
                                    exit = true;
                                }
                            }
                        }
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            let commands = process_event::poll_event_until_simple_keybind(
                                app_state, backend, m,
                            );

                            if let Some(commands) = commands {
                                for command in commands {
                                    if let Command::ShowTasks = command {
                                        exit = true;
                                    }
                                }
                            }
                        }
                    }
                    app_state.flush_event();
                }
                event => process_event::process_noninteractive(event, app_state),
            };
        }
    }
    Ok(())
}
