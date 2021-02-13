use std::iter::Iterator;

use termion::event::{Event, Key};
use tui::layout::Rect;
use tui::widgets::Clear;

use crate::commands::{CommandKeybind, KeyCommand};
use crate::config::JoshutoCommandMapping;
use crate::context::JoshutoContext;
use crate::ui::views::TuiView;
use crate::ui::widgets::TuiMenu;
use crate::ui::TuiBackend;
use crate::util::event::JoshutoEvent;
use crate::util::input;
use crate::util::to_string::ToString;

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
        context: &mut JoshutoContext,
        m: &'a JoshutoCommandMapping,
    ) -> Option<&'a KeyCommand> {
        let mut map: &JoshutoCommandMapping = &m;
        let terminal = backend.terminal_mut();
        context.flush_event();

        loop {
            let _ = terminal.draw(|frame| {
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
                        .map(|(k, v)| format!("  {}        {}", k.to_string(), v))
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
                    JoshutoEvent::Termion(event) => {
                        match event {
                            Event::Key(Key::Esc) => return None,
                            event => match map.as_ref().get(&event) {
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
                    event => input::process_noninteractive(event, context),
                }
            }
        }
    }
}
