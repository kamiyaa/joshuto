use ratatui::termion::event::{Event, Key};

use crate::commands::cursor_move;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::run::process_event;
use crate::traits::app_execute::NumberedExecute;
use crate::types::event::AppEvent;
use crate::types::keybind::CommandKeybind;
use crate::types::keymap::AppKeyMapping;
use crate::types::state::AppState;
use crate::ui::views::TuiView;
use crate::ui::AppBackend;

pub fn numbered_command(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    keymap: &AppKeyMapping,
    first_char: char,
) -> AppResult {
    app_state.flush_event();
    let mut prefix = String::from(first_char);

    loop {
        app_state
            .state
            .message_queue_mut()
            .push_info(prefix.clone());
        backend.render(TuiView::new(app_state));
        app_state.state.message_queue_mut().pop_front();

        let event = match app_state.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        let num_prefix = match prefix.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                app_state.state.message_queue_mut().pop_front();
                return Err(AppError::new(
                    AppErrorKind::Parse,
                    "Number is too big".to_string(),
                ));
            }
        };

        match event {
            AppEvent::TerminalEvent(event) => {
                match event {
                    Event::Key(Key::Esc) => return Ok(()),
                    Event::Key(Key::Char('g')) => {
                        cursor_move::cursor_move(app_state, num_prefix - 1);
                        return Ok(());
                    }
                    Event::Key(Key::Char(c)) if c.is_numeric() => {
                        prefix.push(c);
                    }
                    key => match keymap.default_view.get(&key) {
                        Some(CommandKeybind::SimpleKeybind { commands, .. }) => {
                            for command in commands {
                                let _ = command
                                    .numbered_execute(num_prefix, app_state, backend, keymap);
                            }
                            return Ok(());
                        }
                        _ => {
                            return Err(AppError::new(
                                AppErrorKind::UnrecognizedCommand,
                                "Command cannot be prefixed by a number or does not exist"
                                    .to_string(),
                            ));
                        }
                    },
                }
                app_state.flush_event();
            }
            event => process_event::process_noninteractive(event, app_state),
        }
    }
}
