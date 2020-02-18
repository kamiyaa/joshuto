use std::io::{self, Write};
use std::iter::Iterator;

use tui::buffer::Buffer;
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

use crate::commands::{CommandKeybind, CursorMoveUp, JoshutoCommand, JoshutoRunnable};
use crate::config::JoshutoCommandMapping;
use crate::context::JoshutoContext;
use crate::ui::TuiBackend;
use crate::util::event::{Event, Events};
use super::{TuiDirList, TuiDirListDetailed, TuiTopBar};

use crate::{HOSTNAME, USERNAME};
use super::super::{DEFAULT_LAYOUT, NO_PREVIEW_LAYOUT};

const BORDER_HEIGHT: usize = 1;
const BOTTOM_MARGIN: usize = 1;

pub struct TuiCommandMenu;

impl TuiCommandMenu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_input<'a>(&mut self, backend: &mut TuiBackend,
            context: &JoshutoContext, m: &'a JoshutoCommandMapping) -> 
        Option<&'a Box<JoshutoCommand>> {
        let mut map: &JoshutoCommandMapping = &m;
        let events = &context.events;

        let curr_tab = context.curr_tab_ref();

        let curr_list = curr_tab.curr_list_ref();
        let parent_list = curr_tab.parent_list_ref();
        let child_list = curr_tab.child_list_ref();

        loop {
            backend.terminal.draw(|mut frame| {
                let f_size = frame.size();

                {
                    let top_rect = Rect {
                        x: 0,
                        y: 0,
                        width: f_size.width,
                        height: 1,
                    };

                    TuiTopBar::new(curr_tab.curr_path.as_path())
                        .render(&mut frame, top_rect);
                }

                let constraints = match child_list {
                    Some(_) => DEFAULT_LAYOUT,
                    None => NO_PREVIEW_LAYOUT,
                };
                let layout_rect = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints(constraints.as_ref())
                    .split(f_size);

                if let Some(curr_list) = parent_list.as_ref() {
                    TuiDirList::new(&curr_list).render(&mut frame, layout_rect[0]);
                };

                if let Some(curr_list) = curr_list.as_ref() {
                    TuiDirListDetailed::new(&curr_list).render(&mut frame, layout_rect[1]);
                };

                if let Some(curr_list) = child_list.as_ref() {
                    TuiDirList::new(&curr_list).render(&mut frame, layout_rect[2]);
                };

                {
                    // draw menu
                    let mut display_vec: Vec<String> = map
                        .iter()
                        .map(|(k, v)| {
                            format!("  {:?}    {}", k, v)
                        })
                        .collect();
                    display_vec.sort();
                    let display_str: Vec<&str> =
                        display_vec.iter().map(|v| v.as_str()).collect();

                    let display_str_len = display_str.len();

                    let y = if (f_size.height as usize) < display_str_len + BORDER_HEIGHT + BOTTOM_MARGIN {
                            0
                        } else {
                            f_size.height - (BORDER_HEIGHT + BOTTOM_MARGIN) as u16
                                - display_str_len as u16
                        };

                    let menu_rect = Rect {
                        x: 0,
                        y: y,
                        width: f_size.width,
                        height: (display_str_len + BORDER_HEIGHT) as u16,
                    };

                    TuiMenu::new(&display_str).render(&mut frame, menu_rect);
                }
            });
            if let Ok(event) = events.next() {
                match event {
                    Event::Input(Key::Esc) => {
                        return None;
                    }
                    Event::Input(key) => {
                        match map.get(&key) {
                            Some(CommandKeybind::SimpleKeybind(s)) => {
                                return Some(s);
                            }
                            Some(CommandKeybind::CompositeKeybind(m)) => {
                                map = m;
                            }
                            None => return None,
                        }
                    }
                    _ => {},
                }
            }
        }

    }
}

pub struct TuiMenu<'a> {
    options: &'a Vec<&'a str>,
}

impl<'a> TuiMenu<'a> {
    pub fn new(options: &'a Vec<&str>) -> Self {
        Self { options }
    }
}

impl<'a> Widget for TuiMenu<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let text_iter = self.options.iter().map(|s| Text::raw(*s));
        let block = Block::default()
            .borders(Borders::TOP);

        List::new(text_iter)
            .block(block)
            .draw(area, buf);
    }
}
