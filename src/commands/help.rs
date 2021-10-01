use termion::event::{Event, Key};

use crate::commands::{quit, CommandKeybind, KeyCommand};
use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::error::JoshutoResult;
use crate::event::AppEvent;
use crate::ui::widgets;
use crate::ui::widgets::TuiHelp;
use crate::ui::TuiBackend;

pub fn help_loop(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    keymap_t: &AppKeyMapping,
) -> JoshutoResult<()> {
    context.flush_event();

    let mut offset = 0;
    let mut search_query = String::new();
    let mut sort_by = 1;

    while context.quit == QuitType::DoNot {
        let keymap = if search_query.is_empty() {
            widgets::get_keymap_table(keymap_t, &search_query, sort_by)
        } else {
            widgets::get_keymap_table(keymap_t, &search_query[1..], sort_by)
        };
        backend.render(TuiHelp::new(&keymap, &mut offset, &search_query));

        if !context.worker_context_ref().is_busy() && !context.worker_context_ref().is_empty() {
            context.worker_context_mut().start_next_job();
        }

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        if let AppEvent::Termion(event) = event {
            match event {
                Event::Key(Key::Esc) => break,
                Event::Key(Key::Char('1')) => sort_by = 0,
                Event::Key(Key::Char('2')) => sort_by = 1,
                Event::Key(Key::Char('3')) => sort_by = 2,
                //Event::Key(Key::Char('/')) => search_query.push('/'),
                event => {
                    if let Some(CommandKeybind::SimpleKeybind(command)) =
                        keymap_t.as_ref().get(&event)
                    {
                        match command {
                            KeyCommand::CursorMoveUp(_) => move_offset(&mut offset, -1),
                            KeyCommand::CursorMoveDown(_) => move_offset(&mut offset, 1),
                            KeyCommand::CursorMoveHome => offset = 0,
                            KeyCommand::CursorMoveEnd => offset = 255,
                            KeyCommand::CursorMovePageUp => move_offset(&mut offset, -10),
                            KeyCommand::CursorMovePageDown => move_offset(&mut offset, 10),
                            KeyCommand::CloseTab | KeyCommand::Help => break,
                            KeyCommand::ForceQuit => {
                                eprintln!("quit");
                                #[allow(unused_must_use)]
                                {
                                    quit::force_quit(context);
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
            context.flush_event();
        }
    }

    Ok(())
}

// offset is a u8, so if we make it negative program will fail.
// This function prevents this error
fn move_offset(offset: &mut u8, moving_amount: i8) {
    if moving_amount > 0 {
        *offset += moving_amount as u8;
    } else if moving_amount < 0 {
        if *offset > -moving_amount as u8 {
            *offset -= -moving_amount as u8;
        } else {
            *offset = 0;
        }
    }
}
