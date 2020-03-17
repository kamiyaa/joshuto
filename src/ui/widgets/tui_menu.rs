use std::iter::Iterator;

use termion::event::Key;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Borders, Widget};
use unicode_width::UnicodeWidthStr;

use super::TuiView;
use crate::commands::{CommandKeybind, JoshutoCommand};
use crate::config::JoshutoCommandMapping;
use crate::context::JoshutoContext;
use crate::ui::TuiBackend;
use crate::util::event::Event;

const BORDER_HEIGHT: usize = 1;
const BOTTOM_MARGIN: usize = 1;

pub struct TuiCommandMenu;

impl TuiCommandMenu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_input<'a>(
        &mut self,
        backend: &mut TuiBackend,
        context: &JoshutoContext,
        m: &'a JoshutoCommandMapping,
    ) -> Option<&'a Box<dyn JoshutoCommand>> {
        let mut map: &JoshutoCommandMapping = &m;
        let terminal = backend.terminal_mut();
        context.events.flush();

        loop {
            terminal.draw(|mut frame| {
                let f_size = frame.size();

                {
                    let mut view = TuiView::new(&context);
                    view.render(&mut frame, f_size);
                }

                {
                    // draw menu
                    let mut display_vec: Vec<String> = map
                        .iter()
                        .map(|(k, v)| format!("  {:?}    {}", k, v))
                        .collect();
                    display_vec.sort();
                    let display_str: Vec<&str> = display_vec.iter().map(|v| v.as_str()).collect();

                    let display_str_len = display_str.len();

                    let y = if (f_size.height as usize)
                        < display_str_len + BORDER_HEIGHT + BOTTOM_MARGIN
                    {
                        0
                    } else {
                        f_size.height
                            - (BORDER_HEIGHT + BOTTOM_MARGIN) as u16
                            - display_str_len as u16
                    };

                    let menu_rect = Rect {
                        x: 0,
                        y,
                        width: f_size.width,
                        height: (display_str_len + BORDER_HEIGHT) as u16,
                    };

                    TuiMenu::new(&display_str).render(&mut frame, menu_rect);
                }
            });

            if let Ok(event) = context.events.next() {
                match event {
                    Event::Input(key) => {
                        match key {
                            Key::Esc => return None,
                            key => match map.get(&key) {
                                Some(CommandKeybind::SimpleKeybind(s)) => {
                                    return Some(s);
                                }
                                Some(CommandKeybind::CompositeKeybind(m)) => {
                                    map = m;
                                }
                                None => return None,
                            },
                        }
                        context.events.flush();
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct TuiMenu<'a> {
    options: &'a [&'a str],
}

impl<'a> TuiMenu<'a> {
    pub fn new(options: &'a [&'a str]) -> Self {
        Self { options }
    }

    pub fn len(&self) -> usize {
        self.options.len()
    }
}

const LONG_SPACE: &str = "                                                      ";

impl<'a> Widget for TuiMenu<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let text_iter = self.options.iter();
        let mut block = Block::default().borders(Borders::TOP);

        block.draw(area, buf);

        let style = Style::default();

        let area_x = area.x + 1;
        let area_y = area.y + 1;

        for (i, text) in text_iter.enumerate() {
            let width = text.width();
            buf.set_stringn(area_x, area_y + i as u16, text, width, style);
            buf.set_stringn(
                area_x + width as u16,
                area_y + i as u16,
                LONG_SPACE,
                area.width as usize,
                style,
            );
        }
    }
}
