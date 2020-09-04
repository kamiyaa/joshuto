use tui::buffer::Buffer;
use tui::layout::{Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Paragraph, Widget, Wrap};

use super::{TuiDirList, TuiDirListDetailed, TuiFooter, TuiTabBar, TuiTopBar};
use crate::context::JoshutoContext;

const TAB_VIEW_WIDTH: u16 = 15;

pub struct TuiFolderView<'a> {
    pub context: &'a JoshutoContext,
    pub show_bottom_status: bool,
}

use super::super::{DEFAULT_LAYOUT, NO_PREVIEW_LAYOUT};

impl<'a> TuiFolderView<'a> {
    pub fn new(context: &'a JoshutoContext) -> Self {
        Self {
            context,
            show_bottom_status: true,
        }
    }
}

impl<'a> Widget for TuiFolderView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let f_size = area;

        let curr_tab = self.context.tab_context_ref().curr_tab_ref();

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

        if self.context.tab_context_ref().len() > 1 {
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
            TuiTopBar::new(curr_tab.pwd()).render(rect, buf);

            let rect = Rect {
                x: topbar_width,
                y: 0,
                width: TAB_VIEW_WIDTH,
                height: 1,
            };
            let name = if let Some(ostr) = curr_tab.pwd().file_name() {
                ostr.to_str().unwrap_or("")
            } else {
                ""
            };
            TuiTabBar::new(
                name,
                self.context.tab_context_ref().get_index(),
                self.context.tab_context_ref().len(),
            )
            .render(rect, buf);
        } else {
            let topbar_width = f_size.width;

            let rect = Rect {
                x: 0,
                y: 0,
                width: topbar_width,
                height: 1,
            };
            TuiTopBar::new(curr_tab.pwd()).render(rect, buf);
        }

        if let Some(list) = parent_list.as_ref() {
            TuiDirList::new(&list).render(layout_rect[0], buf);
        };

        if let Some(list) = curr_list.as_ref() {
            TuiDirListDetailed::new(&list).render(layout_rect[1], buf);
            let rect = Rect {
                x: 0,
                y: f_size.height - 1,
                width: f_size.width,
                height: 1,
            };

            let message_style = Style::default().fg(Color::Yellow);

            if self.show_bottom_status {
                /* draw the bottom status bar */
                if let Some(msg) = self.context.worker_msg() {
                    let text = Span::styled(msg, message_style);
                    Paragraph::new(text)
                        .wrap(Wrap { trim: true })
                        .render(rect, buf);
                } else if !self.context.message_queue_ref().is_empty() {
                    let text = Span::styled(&self.context.message_queue_ref()[0], message_style);
                    Paragraph::new(text)
                        .wrap(Wrap { trim: true })
                        .render(rect, buf);
                } else if let Some(entry) = list.get_curr_ref() {
                    TuiFooter::new(entry).render(rect, buf);
                }
            }
        };

        if let Some(list) = child_list.as_ref() {
            TuiDirList::new(&list).render(layout_rect[2], buf);
        };
    }
}
