use std::io::Write;

use tui::buffer::Buffer;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use super::widgets::{TuiDirList, TuiDirListDetailed, TuiFooter, TuiTopBar};
use crate::context::JoshutoContext;

pub struct TuiBackend {
    pub terminal: tui::Terminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>>,
}

use super::{DEFAULT_LAYOUT, NO_PREVIEW_LAYOUT};

impl TuiBackend {
    pub fn new() -> std::io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = tui::Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(Self { terminal })
    }

    pub fn render(&mut self, context: &JoshutoContext) {
        let curr_tab = context.curr_tab_ref();

        let curr_list = curr_tab.curr_list_ref();
        let parent_list = curr_tab.parent_list_ref();
        let child_list = curr_tab.child_list_ref();

        let f_size = {
            let frame = self.terminal.get_frame();
            frame.size()
        };

        self.terminal.draw(|mut frame| {
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
                if let Some(entry) = curr_list.get_curr_ref() {
                    let top_rect = Rect {
                        x: 0,
                        y: f_size.height - 1,
                        width: f_size.width,
                        height: 1,
                    };
                    TuiFooter::new(entry)
                        .render(&mut frame, top_rect);
                }
            };

            if let Some(curr_list) = child_list.as_ref() {
                TuiDirList::new(&curr_list).render(&mut frame, layout_rect[2]);
            };
        });
    }
}

impl Widget for TuiBackend {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {

    }
}
