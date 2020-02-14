use std::io::{self, Write};

use rustyline::completion::{Candidate, Completer, FilenameCompleter, Pair};
use rustyline::line_buffer;

use termion::clear;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

use crate::ui::TuiBackend;
use crate::util::event::{Event, Events};
use crate::window;

use crate::KEYMAP_T;

struct CompletionTracker {
    pub index: usize,
    pub pos: usize,
    pub original: String,
    pub candidates: Vec<Pair>,
}

impl CompletionTracker {
    pub fn new(pos: usize, candidates: Vec<Pair>, original: String) -> Self {
        CompletionTracker {
            index: 0,
            pos,
            original,
            candidates,
        }
    }
}

pub struct TextField<'a> {
    backend: &'a mut TuiBackend,
    events: &'a Events,
}

impl<'a> TextField<'a> {
    pub fn new(backend: &'a mut TuiBackend, events: &'a Events) -> Self {
        Self { backend, events }
    }

    pub fn readline(&mut self) -> Option<String> {
        let mut input_string = String::with_capacity(64);
        let events = self.events;

        // initially, clear the line for textfield and move the cursor there as well
        {
            let f_size = {
                let frame = self.backend.terminal.get_frame();
                frame.size()
            };
            let txt_y = f_size.height;

            let termion_terminal = self.backend.terminal.backend_mut();

            write!(termion_terminal, "{}{}", Goto(1, txt_y), clear::AfterCursor);
        }

        loop {
            let f_size = {
                let frame = self.backend.terminal.get_frame();
                frame.size()
            };
            let txt_y = f_size.height;

            let termion_terminal = self.backend.terminal.backend_mut();

            write!(termion_terminal, "{}", Goto(1, txt_y));

            write!(
                termion_terminal,
                "{}{}",
                input_string,
                Goto(1 + input_string.width() as u16, txt_y)
            );

            io::stdout().flush().ok();

            // Handle input
            if let Ok(event) = events.next() {
                match event {
                    Event::Input(input) => match input {
                        Key::Char('\n') => {
                            break;
                        }
                        Key::Esc => {
                            write!(termion_terminal, "{}{}", Goto(1, txt_y), clear::AfterCursor,);
                            return None;
                        }
                        Key::Backspace => {
                            input_string.pop();
                        }
                        Key::Char(c) => {
                            input_string.push(c);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        eprintln!("You typed: {}", input_string);
        Some(input_string)
    }
}
