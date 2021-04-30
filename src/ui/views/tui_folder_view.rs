use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::context::AppContext;
use crate::ui::widgets::{TuiDirList, TuiDirListDetailed, TuiFooter, TuiTabBar, TuiTopBar};

const TAB_VIEW_WIDTH: u16 = 15;

pub struct TuiFolderView<'a> {
    pub context: &'a AppContext,
    pub show_bottom_status: bool,
}

impl<'a> TuiFolderView<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Self {
            context,
            show_bottom_status: true,
        }
    }
}

impl<'a> Widget for TuiFolderView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let curr_tab = self.context.tab_context_ref().curr_tab_ref();

        let curr_list = curr_tab.curr_list_ref();
        let parent_list = curr_tab.parent_list_ref();
        let child_list = curr_tab.child_list_ref();

        let constraints: &[Constraint; 3] =
            if !self.context.display_options_ref().collapse_preview() {
                &self.context.display_options_ref().default_layout
            } else {
                match child_list {
                    Some(_) => &self.context.display_options_ref().default_layout,
                    None => &self.context.display_options_ref().no_preview_layout,
                }
            };

        let layout_rect = if self.context.display_options_ref().show_borders() {
            let area = Rect {
                y: area.top() + 1,
                height: area.height - 2,
                ..area
            };
            let block = Block::default().borders(Borders::ALL);
            let inner = block.inner(area);
            block.render(area, buf);

            let layout_rect = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints.as_ref())
                .split(inner);

            let block = Block::default().borders(Borders::RIGHT);
            let inner1 = block.inner(layout_rect[0]);
            block.render(layout_rect[0], buf);

            let block = Block::default().borders(Borders::LEFT);
            let inner3 = block.inner(layout_rect[2]);
            block.render(layout_rect[2], buf);

            vec![inner1, layout_rect[1], inner3]
        } else {
            let mut layout_rect = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(1)
                .constraints(constraints.as_ref())
                .split(area);

            layout_rect[0] = Rect {
                width: layout_rect[0].width - 1,
                ..layout_rect[0]
            };
            layout_rect[1] = Rect {
                width: layout_rect[1].width - 1,
                ..layout_rect[1]
            };
            layout_rect
        };

        // render parent view
        if let Some(list) = parent_list.as_ref() {
            TuiDirList::new(&list).render(layout_rect[0], buf);
        };

        // render current view
        if let Some(list) = curr_list.as_ref() {
            TuiDirListDetailed::new(&list).render(layout_rect[1], buf);
            let rect = Rect {
                x: 0,
                y: area.height - 1,
                width: area.width,
                height: 1,
            };

            if self.show_bottom_status {
                let message_style = Style::default().fg(Color::Yellow);
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
                } else {
                    TuiFooter::new(list).render(rect, buf);
                }
            }
        };

        // render preview
        if let Some(list) = child_list.as_ref() {
            TuiDirList::new(&list).render(layout_rect[2], buf);
        };

        let topbar_width = area.width;
        let rect = Rect {
            x: 0,
            y: 0,
            width: topbar_width,
            height: 1,
        };
        TuiTopBar::new(self.context, curr_tab.pwd()).render(rect, buf);

        // render tabs
        if self.context.tab_context_ref().len() > 1 {
            let topbar_width = if area.width > TAB_VIEW_WIDTH {
                area.width - TAB_VIEW_WIDTH
            } else {
                0
            };

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
        }
    }
}
