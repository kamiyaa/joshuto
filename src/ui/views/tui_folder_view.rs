use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::symbols::line::{HORIZONTAL_DOWN, HORIZONTAL_UP};
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::context::AppContext;
use crate::preview::preview_default::PreviewState;
use crate::ui;
use crate::ui::widgets::{
    TuiDirList, TuiDirListDetailed, TuiDirListLoading, TuiFilePreview, TuiFooter, TuiMessage,
    TuiTabBar, TuiTopBar,
};
use crate::ui::PreviewArea;

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
        let preview_context = self.context.preview_context_ref();
        let curr_tab = self.context.tab_context_ref().curr_tab_ref();
        let curr_tab_id = self.context.tab_context_ref().curr_tab_id();
        let curr_tab_cwd = curr_tab.cwd();

        let curr_list = curr_tab.curr_list_ref();
        let child_list = curr_tab.child_list_ref();

        let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());

        let config = self.context.config_ref();
        let display_options = config.display_options_ref();

        let constraints = get_constraints(self.context);
        let is_default_layout = constraints == &display_options.default_layout;

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
            block.render(layout_rect[2], buf);

            // Render inner borders properly.
            {
                let top = area.top();
                let bottom = area.bottom() - 1;
                let left = layout_rect[1].left() - 1;
                let right = layout_rect[2].left();
                let intersections = Intersections {
                    top,
                    bottom,
                    left,
                    right,
                };

                // Won't render intersections if parent view is turned off
                match constraints[0] {
                    Constraint::Ratio(0, _) => (),
                    _ => intersections.render_left(buf),
                }
                if is_default_layout {
                    intersections.render_right(buf);
                }
            }
            layout
        } else {
            let area = Rect {
                y: area.top() + 1,
                height: area.height - 2,
                ..area
            };
            calculate_layout(area, constraints)
        };

        // render parent view
        match constraints[0] {
            Constraint::Ratio(0, _) => (),
            _ => {
                if let Some(list) = curr_tab.parent_list_ref().as_ref() {
                    TuiDirList::new(list, true).render(layout_rect[0], buf);
                }
            }
        }

        // render current view
        if let Some(list) = curr_list.as_ref() {
            TuiDirListDetailed::new(list, display_options, true).render(layout_rect[1], buf);
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
                    TuiFooter::new(list).render(rect, buf);
                }
            }
        } else {
            match curr_tab.history_metadata_ref().get(curr_tab_cwd) {
                Some(PreviewState::Loading) => {
                    TuiDirListLoading::new().render(layout_rect[1], buf);
                }
                Some(PreviewState::Error { message }) => {
                    TuiMessage::new(message, Style::default().fg(Color::Red))
                        .render(layout_rect[1], buf);
                }
                None => {}
            }
        }

        if let Some(list) = child_list.as_ref() {
            TuiDirList::new(list, true).render(layout_rect[2], buf);
        } else if let Some(entry) = curr_entry {
            match curr_tab.history_metadata_ref().get(entry.file_path()) {
                Some(PreviewState::Loading) => {
                    TuiDirListLoading::new().render(layout_rect[2], buf);
                }
                Some(PreviewState::Error { message }) => {
                    TuiMessage::new(message, Style::default().fg(Color::Red))
                        .render(layout_rect[2], buf);
                }
                None => {
                    let preview_area = calculate_preview(self.context, layout_rect[2]);
                    if let Some(preview_area) = preview_area {
                        let area = Rect {
                            x: preview_area.preview_area.x,
                            y: preview_area.preview_area.y,
                            width: preview_area.preview_area.width,
                            height: preview_area.preview_area.height,
                        };
                        if let Some(Some(preview)) =
                            preview_context.get_preview_ref(&preview_area.file_preview_path)
                        {
                            TuiFilePreview::new(preview).render(area, buf);
                        }
                    }
                }
            }
        } else {
            TuiMessage::new("Error loading directory", Style::default().fg(Color::Red))
                .render(layout_rect[2], buf);
        }

        let topbar_width = area.width;
        let rect = Rect {
            x: 0,
            y: 0,
            width: topbar_width,
            height: 1,
        };
        TuiTopBar::new(self.context, curr_tab_cwd).render(rect, buf);

        // render tabs
        if self.context.tab_context_ref().len() > 1 {
            let topbar_width = area.width.saturating_sub(TAB_VIEW_WIDTH);

            let rect = Rect {
                x: topbar_width,
                y: 0,
                width: TAB_VIEW_WIDTH,
                height: 1,
            };
            let name = curr_tab_id.to_string();
            TuiTabBar::new(
                &name[..5],
                self.context.tab_context_ref().index,
                self.context.tab_context_ref().len(),
            )
            .render(rect, buf);
        }
    }
}

struct Intersections {
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
}

impl Intersections {
    fn render_left(&self, buf: &mut Buffer) {
        buf.get_mut(self.left, self.top).set_symbol(HORIZONTAL_DOWN);
        buf.get_mut(self.left, self.bottom)
            .set_symbol(HORIZONTAL_UP);
    }
    fn render_right(&self, buf: &mut Buffer) {
        buf.get_mut(self.right, self.top)
            .set_symbol(HORIZONTAL_DOWN);
        buf.get_mut(self.right, self.bottom)
            .set_symbol(HORIZONTAL_UP);
    }
}

pub fn get_constraints(context: &AppContext) -> &[Constraint; 3] {
    let display_options = context.config_ref().display_options_ref();
    if context.tab_context_ref().len() == 0 {
        return &display_options.default_layout;
    }

    let preview_context = context.preview_context_ref();
    let curr_tab = context.tab_context_ref().curr_tab_ref();

    let curr_list = curr_tab.curr_list_ref();
    let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());

    let child_list = curr_tab.child_list_ref();

    if !display_options.collapse_preview() {
        &display_options.default_layout
    } else {
        match child_list {
            Some(_) => &display_options.default_layout,
            None => match curr_entry {
                None => &display_options.no_preview_layout,
                Some(entry) if entry.metadata.file_type().is_dir() => {
                    &display_options.default_layout
                }
                Some(entry) => match preview_context.get_preview_ref(entry.file_path()) {
                    Some(Some(p)) if p.status.code() != Some(1) => &display_options.default_layout,
                    _ => &display_options.no_preview_layout,
                },
            },
        }
    }
}

pub fn calculate_layout(area: Rect, constraints: &[Constraint; 3]) -> Vec<Rect> {
    let mut layout_rect = Layout::default()
        .direction(Direction::Horizontal)
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
}

pub fn calculate_layout_with_borders(area: Rect, constraints: &[Constraint; 3]) -> Vec<Rect> {
    let block = Block::default().borders(Borders::ALL);
    let inner = block.inner(area);

    let layout_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(inner);

    let block = Block::default().borders(Borders::RIGHT);
    let inner1 = block.inner(layout_rect[0]);

    let block = Block::default().borders(Borders::LEFT);
    let inner3 = block.inner(layout_rect[2]);

    vec![inner1, layout_rect[1], inner3]
}

pub fn calculate_preview(context: &AppContext, rect: Rect) -> Option<PreviewArea> {
    let preview_context = context.preview_context_ref();
    let curr_tab = context.tab_context_ref().curr_tab_ref();

    let child_list = curr_tab.child_list_ref();

    let curr_list = curr_tab.curr_list_ref();
    let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());

    if child_list.as_ref().is_some() {
        None
    } else if let Some(entry) = curr_entry {
        if let Some(Some(preview)) = preview_context.get_preview_ref(entry.file_path()) {
            match preview.status.code() {
                Some(1) | None => None,
                _ => {
                    let file_preview_path = entry.file_path_buf();
                    let preview_area = ui::Rect {
                        x: rect.x,
                        y: rect.y,
                        width: rect.width,
                        height: rect.height,
                    };
                    Some(PreviewArea::new(file_preview_path, preview_area))
                }
            }
        } else {
            None
        }
    } else {
        None
    }
}
