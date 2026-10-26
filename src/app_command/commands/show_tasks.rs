use crate::app_command::AppCommand;
use crate::config::AppKeybindConfig;
use crate::config::KeyEventAction;
use crate::error::AppResult;
use crate::run::process_event;
use crate::state::remove_external_preview;
use crate::state::AppState;
use crate::tasks::AppEvent;
use crate::ui::views::TuiWorkerView;
use crate::ui::AppBackend;
use crate::utils::to_string::ToString;

/// Implements `show_tasks`: runs a nested event loop showing the background-task view until
/// the user presses the key bound back to `show_tasks`.
pub fn show_tasks(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    keymap_t: &AppKeybindConfig,
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
                        Some(KeyEventAction::Base { commands, .. }) => {
                            for command in commands {
                                if let AppCommand::ShowTasks = command {
                                    exit = true;
                                }
                            }
                        }
                        Some(KeyEventAction::Recursive(m)) => {
                            let commands =
                                process_event::poll_event_until_base_keybind(app_state, backend, m);

                            if let Some(commands) = commands {
                                for command in commands {
                                    if let AppCommand::ShowTasks = command {
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
