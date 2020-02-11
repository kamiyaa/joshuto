use std::io::{self, Write};

use rustyline::completion::{Candidate, Completer, FilenameCompleter, Pair};
use rustyline::line_buffer;

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

pub struct TextField<'a, W>
where
    W: std::io::Write,
{
    terminal: &'a mut Terminal<TermionBackend<W>>,
    events: &'a Events,
}

impl<'a, W> TextField<'a, W>
where
    W: std::io::Write,
{
    pub fn new(terminal: &'a mut Terminal<TermionBackend<W>>, events: &'a Events) -> Self {
        Self { terminal, events }
    }
    /*
        Paragraph::new(paragraph_contents.iter())
            .wrap(true)
            .render(&mut f, Rect { x: 0, y: 0, height: 2, width: f_size.width});
    */

    pub fn readline(&mut self) -> Option<String> {
        let mut input_string = String::with_capacity(64);
        let events = self.events;
        loop {
            // Draw UI
            self.terminal.draw(|mut f| {
                let f_size = f.size();
                Paragraph::new([Text::raw(&input_string)].iter())
                    .style(Style::default().fg(Color::Yellow))
                    .render(
                        &mut f,
                        Rect {
                            x: 0,
                            y: 0,
                            height: 2,
                            width: f_size.width,
                        },
                    );
            });

            write!(
                self.terminal.backend_mut(),
                "{}",
                Goto(4 + input_string.width() as u16, 5)
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
