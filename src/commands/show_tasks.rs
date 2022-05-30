use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::event::process_event;
use crate::event::AppEvent;
use crate::key_command::{Command, CommandKeybind};
use crate::ui::views::TuiWorkerView;
use crate::ui::TuiBackend;
use crate::util::to_string::ToString;

pub fn show_tasks(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    keymap_t: &AppKeyMapping,
) -> JoshutoResult {
    context.flush_event();

    loop {
        backend.render(TuiWorkerView::new(context));

        if let Ok(event) = context.poll_event() {
            match event {
                AppEvent::Termion(key) => {
                    match key {
                        key => match keymap_t.task_view.get(&key) {
                            None => {
                                context
                                    .message_queue_mut()
                                    .push_info(format!("Unmapped input: {}", key.to_string()));
                            }
                            Some(CommandKeybind::SimpleKeybind(command)) => match command {
                                Command::ShowTasks => break,
                                _ => {}
                            },
                            Some(CommandKeybind::CompositeKeybind(m)) => {
                                let cmd =
                                    process_event::get_input_while_composite(backend, context, m);

                                if let Some(command) = cmd {
                                    match command {
                                        Command::ShowTasks => break,
                                        _ => {}
                                    }
                                }
                            }
                        },
                    }
                    context.flush_event();
                }
                event => process_event::process_noninteractive(event, context),
            };
        }
    }
    Ok(())
}
