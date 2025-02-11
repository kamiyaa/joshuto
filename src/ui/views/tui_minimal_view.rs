use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Paragraph, Widget, Wrap};

use crate::types::state::AppState;
use crate::ui::widgets::{TuiDirListDetailed, TuiFooter, TuiTopBar};

pub struct TuiMinimalView<'a> {
    pub app_state: &'a AppState,
    pub show_bottom_status: bool,
}

impl<'a> TuiMinimalView<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        Self {
            app_state,
            show_bottom_status: true,
        }
    }
}

impl Widget for TuiMinimalView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tab_state = self.app_state.state.tab_state_ref();

        let display_options = &self.app_state.config.display_options;
        let constraints = &[Constraint::Ratio(1, 1)];

        let layout_rect = {
            let area = Rect {
                y: area.top() + 1,
                height: area.height - 2,
                ..area
            };
            calculate_layout(area, constraints)
        };

        let tab_id = tab_state.curr_tab_id();
        if let Some(curr_tab) = tab_state.tab_ref(&tab_id) {
            let curr_list = curr_tab.curr_list_ref();

            let layout_rect = layout_rect[0];

            // render current view
            if let Some(list) = curr_list.as_ref() {
                TuiDirListDetailed::new(
                    &self.app_state.config,
                    list,
                    display_options,
                    curr_tab.option_ref(),
                    true,
                )
                .render(layout_rect, buf);
                let rect = Rect {
                    x: 0,
                    y: area.height - 1,
                    width: area.width,
                    height: 1,
                };

                if self.show_bottom_status {
                    /* draw the bottom status bar */
                    if let Some(msg) = self.app_state.state.worker_state_ref().get_msg() {
                        let message_style = Style::default().fg(Color::Yellow);
                        let text = Span::styled(msg, message_style);
                        Paragraph::new(text)
                            .wrap(Wrap { trim: true })
                            .render(rect, buf);
                    } else if let Some(msg) =
                        self.app_state.state.message_queue_ref().current_message()
                    {
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
            TuiTopBar::new(self.app_state).render(rect, buf);
        }
    }
}

fn calculate_layout(area: Rect, constraints: &[Constraint; 1]) -> Vec<Rect> {
    let mut layout_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(area)
        .to_vec();

    layout_rect[0] = Rect {
        width: layout_rect[0].width - 1,
        ..layout_rect[0]
    };
    layout_rect
}
