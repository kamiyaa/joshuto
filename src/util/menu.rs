use std::io::{self, Write};
use std::iter::Iterator;

use termion::clear;
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

use crate::ui::TuiBackend;
use crate::util::event::{Event, Events};

pub struct OptionMenu<'a> {
    backend: &'a mut TuiBackend,
    events: &'a Events,
}

impl<'a> OptionMenu<'a> {
    pub fn new(backend: &'a mut TuiBackend, events: &'a Events) -> Self {
        Self { backend, events }
    }

    pub fn get_option(&mut self, options: &[&str]) -> Option<Key> {
        let events = self.events;

        // initially, clear the line for textfield and move the cursor there as well
        let f_size = {
            let frame = self.backend.terminal.get_frame();
            frame.size()
        };
        let txt_y = if f_size.height < options.len() as u16 {
            0
        } else {
            f_size.height - options.len() as u16
        };

        let termion_terminal = self.backend.terminal.backend_mut();

        write!(termion_terminal, "{}", Goto(1, txt_y));
        for (i, option) in options.iter().enumerate() {
            write!(
                termion_terminal,
                "{}{}{}",
                option,
                Goto(1, txt_y + i as u16),
                clear::AfterCursor
            );
        }
        io::stdout().flush().ok();

        loop {
            let event = events.next();
            if let Ok(event) = event {
                match event {
                    Event::Input(input) => match input {
                        Key::Esc => return None,
                        key => return Some(key),
                    },
                    _ => {}
                }
            }
        }
    }
}
