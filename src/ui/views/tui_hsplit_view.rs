use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::context::AppContext;
use crate::ui::widgets::{TuiDirListDetailed, TuiFooter, TuiTabBar, TuiTopBar};

const TAB_VIEW_WIDTH: u16 = 15;

pub struct TuiHSplitView<'a> {
    pub context: &'a AppContext,
    pub show_bottom_status: bool,
}

impl<'a> TuiHSplitView<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Self {
            context,
            show_bottom_status: true,
        }
    }
}

impl<'a> Widget for TuiHSplitView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tab_context = self.context.tab_context_ref();

        let config = self.context.config_ref();
        let display_options = config.display_options_ref();
        let constraints = &[Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)];

        let layout_rect = if display_options.show_borders() {
            let area = Rect {
                y: area.top() + 1,
                height: area.height - 2,
                ..area
            };

            let layout = calculate_layout_with_borders(area, constraints);

            let block = Block::default().borders(Borders::ALL);
            let inner = block.inner(area);
            block.render(area, buf);

            let layout_rect = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints.as_ref())
                .split(inner);

            let block = Block::default().borders(Borders::RIGHT);
            block.render(layout_rect[0], buf);

            let block = Block::default().borders(Borders::LEFT);
            block.render(layout_rect[1], buf);

            layout
        } else {
            let area = Rect {
                y: area.top() + 1,
                height: area.height - 2,
                ..area
            };
            calculate_layout(area, constraints)
        };

        let tab_id = tab_context.curr_tab_id();
        let tab_index = tab_context.index;
        if let Some(curr_tab) = tab_context.tab_ref(&tab_id) {
            let curr_list = curr_tab.curr_list_ref();

            let layout_rect = if tab_index % 2 == 0 {
                layout_rect[0]
            } else {
                layout_rect[1]
            };

            // render current view
            if let Some(list) = curr_list.as_ref() {
                TuiDirListDetailed::new(list, display_options, curr_tab.option_ref(), true)
                    .render(layout_rect, buf);
                let rect = Rect {
                    x: 0,
                    y: area.height - 1,
                    width: area.width,
                    height: 1,
                };

                if self.show_bottom_status {
                    /* draw the bottom status bar */
                    if let Some(msg) = self.context.worker_context_ref().get_msg() {
                        let message_style = Style::default().fg(Color::Yellow);
                        let text = Span::styled(msg, message_style);
                        Paragraph::new(text)
                            .wrap(Wrap { trim: true })
                            .render(rect, buf);
                    } else if let Some(msg) = self.context.message_queue_ref().current_message() {
                        let text = Span::styled(msg.content.as_str(), msg.style);
                        Paragraph::new(text)
                            .wrap(Wrap { trim: true })
                            .render(rect, buf);
                    } else {
                        TuiFooter::new(list, curr_tab.option_ref()).render(rect, buf);
                    }
                }
            }

            let topbar_width = area.width;
            let rect = Rect {
                x: 0,
                y: 0,
                width: topbar_width,
                height: 1,
            };
            TuiTopBar::new(self.context, curr_tab.cwd()).render(rect, buf);

            // render tabs
            if self.context.tab_context_ref().len() > 1 {
                let topbar_width = area.width.saturating_sub(TAB_VIEW_WIDTH);

                let rect = Rect {
                    x: topbar_width,
                    y: 0,
                    width: TAB_VIEW_WIDTH,
                    height: 1,
                };
                TuiTabBar::new(self.context.tab_context_ref()).render(rect, buf);
            }
        }

        let other_tab_index = if tab_index % 2 == 0 {
            tab_index + 1
        } else {
            tab_index - 1
        };

        if other_tab_index < tab_context.tab_order.len() {
            let other_tab_id = tab_context.tab_order[other_tab_index];
            if let Some(curr_tab) = tab_context.tab_ref(&other_tab_id) {
                let curr_list = curr_tab.curr_list_ref();

                let layout_rect = if other_tab_index % 2 == 0 {
                    layout_rect[0]
                } else {
                    layout_rect[1]
                };

                if let Some(list) = curr_list.as_ref() {
                    TuiDirListDetailed::new(list, display_options, curr_tab.option_ref(), false)
                        .render(layout_rect, buf);
                }
            }
        }
    }
}

fn calculate_layout(area: Rect, constraints: &[Constraint; 2]) -> Vec<Rect> {
    let mut layout_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(area)
        .to_vec();

    layout_rect[0] = Rect {
        width: layout_rect[0].width - 1,
        ..layout_rect[0]
    };
    layout_rect[1] = Rect {
        width: layout_rect[1].width - 1,
        ..layout_rect[1]
    };
    layout_rect
}

fn calculate_layout_with_borders(area: Rect, constraints: &[Constraint; 2]) -> Vec<Rect> {
    let block = Block::default().borders(Borders::ALL);
    let inner = block.inner(area);

    let layout_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(inner);

    let block = Block::default().borders(Borders::RIGHT);
    let inner1 = block.inner(layout_rect[0]);

    let block = Block::default().borders(Borders::LEFT);
    let inner2 = block.inner(layout_rect[1]);

    vec![inner1, inner2]
}
