use tui::buffer::Buffer;
use tui::layout::{Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Paragraph, Text, Widget};

use super::{TuiDirList, TuiDirListDetailed, TuiFooter, TuiTabBar, TuiTopBar};
use crate::context::JoshutoContext;

const TAB_VIEW_WIDTH: u16 = 15;

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
        let f_size = area;

        let curr_tab = self.context.curr_tab_ref();

        let curr_list = curr_tab.curr_list_ref();
        let parent_list = curr_tab.parent_list_ref();
        let child_list = curr_tab.child_list_ref();

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
            let curr_path = curr_tab.curr_path.as_path();

            if self.context.tabs.len() > 1 {
                let topbar_width = if f_size.width > TAB_VIEW_WIDTH {
                    f_size.width - TAB_VIEW_WIDTH
                } else {
                    0
                };

                let rect = Rect {
                    x: 0,
                    y: 0,
                    width: topbar_width,
                    height: 1,
                };
                TuiTopBar::new(curr_path).draw(rect, buf);

                let rect = Rect {
                    x: topbar_width,
                    y: 0,
                    width: TAB_VIEW_WIDTH,
                    height: 1,
                };
                let name = if let Some(ostr) = curr_path.file_name() {
                    ostr.to_str().unwrap_or("")
                } else {
                    ""
                };
                TuiTabBar::new(name, self.context.curr_tab_index, self.context.tabs.len())
                    .draw(rect, buf);
            } else {
                let topbar_width = f_size.width;

                let rect = Rect {
                    x: 0,
                    y: 0,
                    width: topbar_width,
                    height: 1,
                };
                TuiTopBar::new(curr_path).draw(rect, buf);
            }
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

            let message_style = Style::default().fg(Color::Yellow);

            if self.show_bottom_status {
                /* draw the bottom status bar */
                if !self.context.message_queue.is_empty() {
                    let text = [Text::styled(&self.context.message_queue[0], message_style)];

                    Paragraph::new(text.iter()).wrap(true).draw(rect, buf);
                } else if let Some(msg) = self.context.worker_msg.as_ref() {
                    let text = [Text::styled(msg, message_style)];

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
