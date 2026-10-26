use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Widget};

use crate::global;
use crate::state::AppState;
use crate::ui::widgets::{TuiDirListDetailed, TuiStatusBar, TuiTopBar};

/// A two-pane layout showing the current and next tab's listings side by side (paired by even
/// and odd tab index).
pub struct TuiHSplitView<'a> {
    pub app_state: &'a AppState,
    pub show_bottom_status: bool,
}

impl<'a> TuiHSplitView<'a> {
    /// Creates the horizontal-split view for `app_state`.
    pub fn new(app_state: &'a AppState) -> Self {
        Self {
            app_state,
            show_bottom_status: true,
        }
    }
}

impl Widget for TuiHSplitView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tab_state = &self.app_state.state.tab_state;

        let display_options = &self.app_state.config.display_options;
        let constraints = &[Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)];

        let layout_rect = if display_options.show_borders {
            let area = Rect {
                y: area.top() + 1,
                height: area.height - 2,
                ..area
            };

            let layout = calculate_layout_with_borders(area, constraints);

            let border_style = global::APP_THEME.border.as_style();

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border_style);
            let inner = block.inner(area);
            block.render(area, buf);

            let layout_rect = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(inner);

            let block = Block::default()
                .borders(Borders::RIGHT)
                .border_style(border_style);
            block.render(layout_rect[0], buf);

            let block = Block::default()
                .borders(Borders::LEFT)
                .border_style(border_style);
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

        let tab_id = tab_state.curr_tab_id();
        let tab_index = tab_state.index;
        if let Some(curr_tab) = tab_state.tabs.get(&tab_id) {
            let curr_list = curr_tab.curr_list_ref();

            let layout_rect = if tab_index.is_multiple_of(2) {
                layout_rect[0]
            } else {
                layout_rect[1]
            };

            // render current view
            if let Some(list) = curr_list.as_ref() {
                TuiDirListDetailed::new(
                    &self.app_state.config,
                    list,
                    display_options,
                    &curr_tab.options,
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
                    TuiStatusBar::new(self.app_state, list, &curr_tab.options).render(rect, buf);
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

        let other_tab_index = if tab_index.is_multiple_of(2) {
            tab_index + 1
        } else {
            tab_index - 1
        };

        if other_tab_index < tab_state.tab_order.len() {
            let other_tab_id = tab_state.tab_order[other_tab_index];
            if let Some(curr_tab) = tab_state.tabs.get(&other_tab_id) {
                let curr_list = curr_tab.curr_list_ref();

                let layout_rect = if other_tab_index.is_multiple_of(2) {
                    layout_rect[0]
                } else {
                    layout_rect[1]
                };

                if let Some(list) = curr_list.as_ref() {
                    TuiDirListDetailed::new(
                        &self.app_state.config,
                        list,
                        display_options,
                        &curr_tab.options,
                        false,
                    )
                    .render(layout_rect, buf);
                }
            }
        }
    }
}

fn calculate_layout(area: Rect, constraints: &[Constraint; 2]) -> Vec<Rect> {
    let mut layout_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
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
        .constraints(constraints)
        .split(inner);

    let block = Block::default().borders(Borders::RIGHT);
    let inner1 = block.inner(layout_rect[0]);

    let block = Block::default().borders(Borders::LEFT);
    let inner2 = block.inner(layout_rect[1]);

    vec![inner1, inner2]
}
