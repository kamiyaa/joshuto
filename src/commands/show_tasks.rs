use crate::config::clean::keymap::AppKeyMapping;
use crate::context::AppContext;
use crate::error::AppResult;
use crate::event::process_event;
use crate::event::AppEvent;
use crate::key_command::{Command, CommandKeybind};
use crate::traits::ToString;
use crate::ui::views::TuiWorkerView;
use crate::ui::AppBackend;

pub fn show_tasks(
    context: &mut AppContext,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
) -> AppResult {
    context.flush_event();

    let mut exit = false;

    while !exit {
        backend.render(TuiWorkerView::new(context));

        if let Ok(event) = context.poll_event() {
            match event {
                AppEvent::Termion(key) => {
                    let key = key;
                    match keymap_t.task_view.get(&key) {
                        None => {
                            context
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
                            let commands =
                                process_event::poll_event_until_simple_keybind(backend, context, m);

                            if let Some(commands) = commands {
                                for command in commands {
                                    if let Command::ShowTasks = command {
                                        exit = true;
                                    }
                                }
                            }
                        }
                    }
                    context.flush_event();
                }
                event => process_event::process_noninteractive(event, context),
            };
        }
    }
    Ok(())
}
