use std::iter::Iterator;

use termion::event::{Event, Key};
use tui::layout::Rect;
use tui::widgets::Clear;

use crate::config::AppBookmarkMapping;
use crate::context::AppContext;
use crate::ui::views::TuiView;
use crate::ui::widgets::TuiMenu;
use crate::ui::TuiBackend;
use crate::util::event::AppEvent;
use crate::util::input;
// use crate::util::to_string::ToString;

const BORDER_HEIGHT: usize = 1;
const BOTTOM_MARGIN: usize = 1;

type P = std::path::PathBuf;

// fn notify<T: std::fmt::Debug>(x: T){
//     let log = format!("{:?}", x);
//     let _  = std::process::Command::new("notify-send").arg(log).status();
// }
pub struct TuiBookmarkMenu;

// TODO    lifetimes
impl TuiBookmarkMenu {
    pub fn new() -> Self {
        Self
    }

    pub fn get_bm<'a>(
        &mut self,
        backend: &mut TuiBackend,
        context: &'a mut AppContext,
        // map: &'a AppBookmarkMapping,
    ) -> Option<&'a P> {
        let terminal = backend.terminal_mut();
        context.flush_event();

        let _ = terminal.draw(|frame| {
            let f_size: Rect = frame.size();

            {
                let view = TuiView::new(&context);
                frame.render_widget(view, f_size);
            }

            {
                // draw menu
                let display_vec: Vec<String> = context
                    .bookmarks
                    .map
                    .iter()
                    // .map(|(k, v)| format!("  {:?}        {}", k, v))
                    .map(|(k, v)| match k {
                        Event::Key(Key::Char(c)) => format!("  {}        {:?}", c, v.as_path()),
                        _ => "???".to_string(),
                    })
                    .collect();
                // display_vec.sort();
                let display_str: Vec<&str> = display_vec.iter().map(|v| v.as_str()).collect();
                let display_str_len = display_str.len();

                let y = if (f_size.height as usize)
                    < display_str_len + BORDER_HEIGHT + BOTTOM_MARGIN
                {
                    0
                } else {
                    f_size.height - (BORDER_HEIGHT + BOTTOM_MARGIN) as u16 - display_str_len as u16
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
                AppEvent::Termion(event) => {
                    match event {
                        Event::Key(Key::Esc) => return None,
                        event => match context.bookmarks.map.get(&event) {
                            Some(path) => {
                                return Some(path);
                            }
                            None => return None,
                        },
                    }
                    context.flush_event();
                }
                event => input::process_noninteractive(event, context),
            }
        }
        None
    }

    pub fn get_any_char<'a>(
        &mut self,
        backend: &mut TuiBackend,
        context: &'a mut AppContext,
    ) -> Option<Event> {
        let terminal = backend.terminal_mut();
        context.flush_event();

        let _ = terminal.draw(|frame| {
            let f_size: Rect = frame.size();

            {
                let view = TuiView::new(&context);
                frame.render_widget(view, f_size);
            }

            {
                // draw menu
                // let mut display_vec: Vec<String> = context.bookmarks.map
                let display_vec: Vec<String> = vec!["<any>".to_string()];
                // display_vec.sort();
                let display_str: Vec<&str> = display_vec.iter().map(|v| v.as_str()).collect();
                let display_str_len = display_str.len();

                let y = if (f_size.height as usize)
                    < display_str_len + BORDER_HEIGHT + BOTTOM_MARGIN
                {
                    0
                } else {
                    f_size.height - (BORDER_HEIGHT + BOTTOM_MARGIN) as u16 - display_str_len as u16
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

        // if let Ok(event) = context.poll_event() {
        //     match event {
        //         AppEvent::Termion(Event::Key(Key::Esc)) =>return None,
        //         AppEvent::Termion(Event::Key(Key::Char(c))) =>return Some(Event::Key(Key::Char(c))),
        //         event =>return None,

        //             }
        //             context.flush_event();
        //         }
        //         event => input::process_noninteractive(event, context);
        //     }
        // }

        if let Ok(event) = context.poll_event() {
            match event {
                AppEvent::Termion(event) => {
                    match event {
                        Event::Key(Key::Esc) => return None,
                        Event::Key(Key::Char(c)) => return Some(Event::Key(Key::Char(c))),

                        event => return None,
                    }
                    context.flush_event();
                }
                event => input::process_noninteractive(event, context),
            }
        }

        None
    }
}
