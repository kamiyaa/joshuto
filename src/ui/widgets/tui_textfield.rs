use rustyline::completion::{Candidate, Completer, FilenameCompleter, Pair};
use rustyline::line_buffer;

use termion::clear;
use termion::cursor::Goto;
use termion::event::Key;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use unicode_width::UnicodeWidthStr;

use crate::context::JoshutoContext;
use crate::ui::TuiBackend;
use crate::util::event::{Event, Events};

use super::TuiMenu;

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

pub struct TuiTextField<'a> {
    menu: Option<&'a mut TuiMenu<'a>>,
}

impl<'a> TuiTextField<'a> {
    pub fn new(menu: &'a mut TuiMenu<'a>) -> Self {
        Self { menu: Some(menu) }
    }

    pub fn get_input(
        &mut self,
        backend: &mut TuiBackend,
        context: &JoshutoContext,
    ) -> Option<String> {
        let mut input_string = String::with_capacity(64);

        loop {
            backend.terminal.draw(|mut frame| {
                let f_size = frame.size();

                let top_rect = Rect {
                    x: 0,
                    y: 0,
                    width: f_size.width,
                    height: 1,
                };

                if let Some(menu) = self.menu.as_mut() {
                    menu.render(&mut frame, top_rect);
                }
            });

            if let Ok(event) = context.events.next() {
                match event {
                    Event::Input(Key::Esc) => {
                        return None;
                    }
                    Event::Input(Key::Char('\n')) => {
                        break;
                    }
                    Event::Input(Key::Char(c)) => {
                        input_string.push(c);
                    }
                    _ => {}
                };
            }
        }
        eprintln!("You typed: {}", input_string);
        Some(input_string)
    }
}
