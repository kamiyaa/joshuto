use std::iter::Iterator;

use termion::event::Key;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Clear, Widget};

use super::TuiView;
use crate::commands::{CommandKeybind, KeyCommand};
use crate::config::JoshutoCommandMapping;
use crate::context::JoshutoContext;
use crate::ui::TuiBackend;
use crate::util::event::Event;
use crate::util::worker;

const BORDER_HEIGHT: usize = 1;
const BOTTOM_MARGIN: usize = 1;

pub struct TuiCommandMenu;

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Key {
    fn to_string(&self) -> String {
        match *self {
            Key::Char(c) => format!("{}", c),
            Key::Ctrl(c) => format!("ctrl+{}", c),
            Key::Left => format!("arrow_left"),
            Key::Right => format!("arrow_right"),
            Key::Up => format!("arrow_up"),
            Key::Down => format!("arrow_down"),
            Key::Backspace => format!("backspace"),
            Key::Home => format!("home"),
            Key::End => format!("end"),
            Key::PageUp => format!("page_up"),
            Key::PageDown => format!("page_down"),
            Key::BackTab => format!("backtab"),
            Key::Insert => format!("insert"),
            Key::Delete => format!("delete"),
            Key::Esc => format!("escape"),
            Key::F(i) => format!("f{}", i),
            k => format!("{:?}", k),
        }
    }
}

impl TuiCommandMenu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_input<'a>(
        &mut self,
        backend: &mut TuiBackend,
        context: &mut JoshutoContext,
        m: &'a JoshutoCommandMapping,
    ) -> Option<&'a KeyCommand> {
        let mut map: &JoshutoCommandMapping = &m;
        let terminal = backend.terminal_mut();
        context.flush_event();

        loop {
            terminal.draw(|frame| {
                let f_size: Rect = frame.size();

                {
                    let view = TuiView::new(&context);
                    frame.render_widget(view, f_size);
                }

                {
                    // draw menu
                    let mut display_vec: Vec<String> = map
                        .as_ref()
                        .iter()
                        .map(|(k, v)| format!("  {}    {}", k.to_string(), v))
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

                    frame.render_widget(Clear, menu_rect);
                    frame.render_widget(TuiMenu::new(&display_str), menu_rect);
                }
            });

            if let Ok(event) = context.poll_event() {
                match event {
                    Event::IOWorkerProgress(res) => {
                        worker::process_worker_progress(context, res);
                    }
                    Event::IOWorkerResult(res) => {
                        worker::process_finished_worker(context, res);
                    }
                    Event::Input(key) => {
                        match key {
                            Key::Esc => return None,
                            key => match map.as_ref().get(&key) {
                                Some(CommandKeybind::SimpleKeybind(s)) => {
                                    return Some(s);
                                }
                                Some(CommandKeybind::CompositeKeybind(m)) => {
                                    map = m;
                                }
                                None => return None,
                            },
                        }
                        context.flush_event();
                    }
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

impl<'a> Widget for TuiMenu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text_iter = self.options.iter().chain(&[" "]);
        let style = Style::default().fg(Color::Reset).bg(Color::Reset);
        let area_x = area.x + 1;
        let area_y = area.y + 1;

        Block::default()
            .style(style)
            .borders(Borders::TOP)
            .render(area, buf);

        for (i, text) in text_iter.enumerate() {
            buf.set_string(area_x, area_y + i as u16, text, style);
        }
    }
}
