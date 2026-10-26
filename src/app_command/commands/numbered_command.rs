use ratatui::termion::event::{Event, Key};

use crate::app_command::commands::cursor_move;
use crate::app_command::NumberPrefixCommand;
use crate::config::AppKeybindConfig;
use crate::config::KeyEventAction;
use crate::error::{AppError, AppErrorCode, AppResult};
use crate::run::process_event;
use crate::state::{AppState, Message};
use crate::tasks::AppEvent;
use crate::ui::views::TuiView;
use crate::ui::AppBackend;

/// Implements `numbered_command`: reads further digits to build a count prefix, then dispatches
/// the next bound command via [`NumberPrefixCommand`] (or jumps to that line number on `g`).
pub fn numbered_command(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    keymap: &AppKeybindConfig,
    first_char: char,
) -> AppResult {
    app_state.flush_event();
    let mut prefix = String::from(first_char);

    loop {
        // shown ahead of any queued messages, and removed once the next event arrives
        app_state
            .state
            .message_queue
            .push_front(Message::info(prefix.clone()));
        backend.render(TuiView::new(app_state));

        let event = app_state.poll_event();
        app_state.state.message_queue.pop_front();
        let event = match event {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        let num_prefix = match prefix.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Err(AppError::new(
                    AppErrorCode::Parse,
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
                        Some(KeyEventAction::Base { commands, .. }) => {
                            for command in commands {
                                let _ = command
                                    .numbered_execute(num_prefix, app_state, backend, keymap);
                            }
                            return Ok(());
                        }
                        _ => {
                            return Err(AppError::new(
                                AppErrorCode::UnrecognizedCommand,
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
