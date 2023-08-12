use std::cmp::Ordering;

use termion::event::{Event, Key};

use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::event::process_event;
use crate::event::AppEvent;
use crate::key_command::{Command, CommandKeybind};
use crate::ui::widgets;
use crate::ui::widgets::TuiHelp;
use crate::ui::AppBackend;

pub fn help_loop(
    context: &mut AppContext,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
) -> JoshutoResult {
    context.flush_event();

    let mut offset = 0;
    let mut search_query = String::new();
    let mut sort_by = 1;

    loop {
        let keymap = if search_query.is_empty() {
            widgets::get_keymap_table(&keymap_t.default_view, &search_query, sort_by)
        } else {
            widgets::get_keymap_table(&keymap_t.default_view, &search_query[1..], sort_by)
        };

        context.remove_external_preview();
        backend.render(TuiHelp::new(&keymap, &mut offset, &search_query));

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        match event {
            AppEvent::Termion(event) => {
                if search_query.is_empty() {
                    match event {
                        Event::Key(Key::Esc) => break,
                        Event::Key(Key::Char('1')) => sort_by = 0,
                        Event::Key(Key::Char('2')) => sort_by = 1,
                        Event::Key(Key::Char('3')) => sort_by = 2,
                        Event::Key(Key::Char('/')) => search_query.push('/'),
                        event => {
                            if let Some(CommandKeybind::SimpleKeybind { command, .. }) =
                                keymap_t.help_view.get(&event)
                            {
                                match command {
                                    Command::CursorMoveUp { .. } => move_offset(&mut offset, -1),
                                    Command::CursorMoveDown { .. } => move_offset(&mut offset, 1),
                                    Command::CursorMoveHome => offset = 0,
                                    Command::CursorMoveEnd => offset = 255,
                                    Command::CursorMovePageUp(_) => move_offset(&mut offset, -10),
                                    Command::CursorMovePageDown(_) => move_offset(&mut offset, 10),
                                    Command::CloseTab | Command::Help => break,
                                    _ => (),
                                }
                            }
                        }
                    }
                } else {
                    match event {
                        Event::Key(Key::Esc) => search_query.clear(),
                        Event::Key(Key::Backspace) => {
                            search_query.pop();
                        }
                        Event::Key(Key::Char(chr)) => search_query.push(chr),
                        _ => (),
                    }
                }
                context.flush_event();
            }
            _ => process_event::process_noninteractive(event, context),
        }
    }

    Ok(())
}

// offset is a u8, so if we make it negative program will fail.
// This function prevents this error
fn move_offset(offset: &mut u8, moving_amount: i8) {
    match moving_amount.cmp(&0) {
        Ordering::Greater => {
            *offset += moving_amount as u8;
        }
        Ordering::Less => {
            if *offset > -moving_amount as u8 {
                *offset -= -moving_amount as u8;
            } else {
                *offset = 0;
            }
        }
        Ordering::Equal => (),
    }
}
