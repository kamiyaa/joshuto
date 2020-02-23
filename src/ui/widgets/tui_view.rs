use tui::buffer::Buffer;
use tui::layout::{Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Paragraph, Text, Widget};
use unicode_width::UnicodeWidthStr;

use super::{TuiDirList, TuiDirListDetailed, TuiFooter, TuiTopBar};
use crate::context::JoshutoContext;

pub struct TuiView<'a> {
    pub context: &'a JoshutoContext,
    pub show_bottom_status: bool,
}

use super::super::{DEFAULT_LAYOUT, NO_PREVIEW_LAYOUT};

impl<'a> TuiView<'a> {
    pub fn new(context: &'a JoshutoContext) -> Self {
        Self {
            context,
            show_bottom_status: true,
        }
    }
}

impl<'a> Widget for TuiView<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let curr_tab = self.context.curr_tab_ref();

        let curr_list = curr_tab.curr_list_ref();
        let parent_list = curr_tab.parent_list_ref();
        let child_list = curr_tab.child_list_ref();

        let f_size = area;

        let constraints = match child_list {
            Some(_) => DEFAULT_LAYOUT,
            None => NO_PREVIEW_LAYOUT,
        };
        let layout_rect = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(constraints.as_ref())
            .split(f_size);

        {
            let rect = Rect {
                x: 0,
                y: 0,
                width: f_size.width,
                height: 1,
            };

            TuiTopBar::new(curr_tab.curr_path.as_path()).draw(rect, buf);
        }

        if let Some(curr_list) = parent_list.as_ref() {
            TuiDirList::new(&curr_list).draw(layout_rect[0], buf);
        };

        if let Some(curr_list) = curr_list.as_ref() {
            TuiDirListDetailed::new(&curr_list).draw(layout_rect[1], buf);
            let rect = Rect {
                x: 0,
                y: f_size.height - 1,
                width: f_size.width,
                height: 1,
            };

            let message_style = Style::default()
                .fg(Color::LightCyan)
                .modifier(Modifier::BOLD);

            if self.show_bottom_status {
                /* draw the bottom status bar */
                if let Some(msg) = self.context.worker_msg.as_ref() {
                    let text = [Text::styled(msg, message_style)];

                    Paragraph::new(text.iter()).wrap(true).draw(rect, buf);
                } else if !self.context.message_queue.is_empty() {
                    let text = [Text::styled(&self.context.message_queue[0], message_style)];

                    Paragraph::new(text.iter()).wrap(true).draw(rect, buf);
                } else if let Some(entry) = curr_list.get_curr_ref() {
                    TuiFooter::new(entry).draw(rect, buf);
                }
            }
        };

        if let Some(curr_list) = child_list.as_ref() {
            TuiDirList::new(&curr_list).draw(layout_rect[2], buf);
        };
    }
}
