use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::context::JoshutoContext;
use crate::ui::widgets::{TuiDirList, TuiDirListDetailed, TuiFooter, TuiTabBar, TuiTopBar};

const TAB_VIEW_WIDTH: u16 = 15;

pub struct TuiFolderView<'a> {
    pub context: &'a JoshutoContext,
    pub show_bottom_status: bool,
    pub show_borders: bool,
    pub collapse_preview: bool,
}

impl<'a> TuiFolderView<'a> {
    pub fn new(context: &'a JoshutoContext) -> Self {
        Self {
            context,
            collapse_preview: context.config_ref().collapse_preview,
            show_bottom_status: true,
            show_borders: context.config_ref().show_borders,
        }
    }
}

impl<'a> Widget for TuiFolderView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let curr_tab = self.context.tab_context_ref().curr_tab_ref();

        let curr_list = curr_tab.curr_list_ref();
        let parent_list = curr_tab.parent_list_ref();
        let child_list = curr_tab.child_list_ref();

        let constraints: &[Constraint; 3] = if !self.collapse_preview {
            &self.context.config_ref().default_layout
        } else {
            match child_list {
                Some(_) => &self.context.config_ref().default_layout,
                None => &self.context.config_ref().no_preview_layout,
            }
        };

        let layout_rect = if self.show_borders {
            let area = Rect {
                x: area.left(),
                y: area.top() + 1,
                width: area.right(),
                height: area.bottom() - 2,
            };
            let block = Block::default().borders(Borders::ALL);
            let inner = block.inner(area.clone());
            block.render(area.clone(), buf);

            let layout_rect = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints.as_ref())
                .split(inner);

            let block = Block::default().borders(Borders::RIGHT);
            let inner1 = block.inner(layout_rect[0].clone());
            block.render(layout_rect[0].clone(), buf);

            let block = Block::default().borders(Borders::LEFT);
            let inner3 = block.inner(layout_rect[2].clone());
            block.render(layout_rect[2].clone(), buf);

            vec![inner1, layout_rect[1], inner3]
        } else {
            Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(1)
                .constraints(constraints.as_ref())
                .split(area)
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
                } else {
                    TuiFooter::new(list).render(rect, buf);
                }
            }
        };

        // render preview
        if let Some(list) = child_list.as_ref() {
            TuiDirList::new(&list).render(layout_rect[2], buf);
        };

        // render tabs
        if self.context.tab_context_ref().len() > 1 {
            let topbar_width = if area.width > TAB_VIEW_WIDTH {
                area.width - TAB_VIEW_WIDTH
            } else {
                0
            };

            let rect = Rect {
                x: 0,
                y: 0,
                width: topbar_width,
                height: 1,
            };
            TuiTopBar::new(self.context, curr_tab.pwd()).render(rect, buf);

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
            let topbar_width = area.width;

            let rect = Rect {
                x: 0,
                y: 0,
                width: topbar_width,
                height: 1,
            };
            TuiTopBar::new(self.context, curr_tab.pwd()).render(rect, buf);
        }
    }
}
