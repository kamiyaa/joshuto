use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::symbols::line::{HORIZONTAL_DOWN, HORIZONTAL_UP};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::context::AppContext;
use crate::preview::preview_dir::PreviewDirState;
use crate::preview::preview_file::PreviewFileState;
use crate::ui;
use crate::ui::widgets::{
    TuiDirList, TuiDirListDetailed, TuiDirListLoading, TuiFilePreview, TuiFooter, TuiMessage,
    TuiTabBar, TuiTopBar,
};
use crate::ui::PreviewArea;

struct TuiFolderViewBorders<'a> {
    pub constraints: &'a [Constraint; 3],
}

impl<'a> TuiFolderViewBorders<'a> {
    pub fn new(constraints: &'a [Constraint; 3]) -> Self {
        Self { constraints }
    }
}

impl<'a> Widget for TuiFolderViewBorders<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL);
        let inner = block.inner(area);
        block.render(area, buf);

        let layout_rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(self.constraints.as_ref())
            .split(inner);

        // Won't render intersections if parent view is turned off
        match self.constraints[0] {
            Constraint::Ratio(0, _) => {}
            _ => {
                let block = Block::default().borders(Borders::RIGHT);
                block.render(layout_rect[0], buf);
            }
        }
        let block = Block::default().borders(Borders::LEFT);
        block.render(layout_rect[2], buf);

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
        match self.constraints[0] {
            Constraint::Ratio(0, _) => (),
            _ => intersections.render_left(buf),
        }
        // Won't render intersections if child preview is unavailable
        match self.constraints[2] {
            Constraint::Ratio(0, _) => (),
            _ => intersections.render_right(buf),
        }
    }
}

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

    pub fn folder_area(area: &Rect) -> Rect {
        Rect {
            y: area.top() + 1,
            height: area.bottom() - 2,
            ..*area
        }
    }

    pub fn header_area(area: &Rect) -> Rect {
        Rect {
            x: area.left(),
            y: area.top(),
            width: area.width,
            height: 1,
        }
    }

    pub fn footer_area(area: &Rect) -> Rect {
        Rect {
            x: area.x,
            y: area.bottom() - 1,
            width: area.width,
            height: 1,
        }
    }

    pub fn tab_area(&self, area: &Rect, num_tabs: usize) -> Rect {
        // render tabs
        let tab_width = (num_tabs * 8) as u16;
        let tab_width = if tab_width > area.width {
            area.width
        } else {
            tab_width
        };
        let topbar_x = area.width.saturating_sub(tab_width);

        Rect {
            x: topbar_x,
            y: area.top(),
            width: tab_width,
            height: 1,
        }
    }
}

impl<'a> Widget for TuiFolderView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let config = self.context.config_ref();
        let display_options = config.display_options_ref();

        let preview_context = self.context.preview_context_ref();
        let curr_tab = self.context.tab_context_ref().curr_tab_ref();
        let curr_tab_cwd = curr_tab.cwd();

        let curr_list = curr_tab.curr_list_ref();
        let child_list = curr_tab.child_list_ref();

        let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());

        let constraints = get_constraints(self.context);

        let layout_rect = if display_options.show_borders() {
            let area = Self::folder_area(&area);
            TuiFolderViewBorders::new(constraints).render(area, buf);

            calculate_layout_with_borders(area, constraints)
        } else {
            let area = Self::folder_area(&area);
            calculate_layout(area, constraints)
        };

        // render parent view
        match constraints[0] {
            Constraint::Ratio(0, _) => {}
            _ => {
                if let Some(list) = curr_tab.parent_list_ref().as_ref() {
                    TuiDirList::new(list, true).render(layout_rect[0], buf);
                }
            }
        }

        // render current view
        if let Some(list) = curr_list.as_ref() {
            TuiDirListDetailed::new(list, display_options, curr_tab.option_ref(), true)
                .render(layout_rect[1], buf);

            let footer_area = Self::footer_area(&area);
            if self.show_bottom_status {
                /* draw the bottom status bar */
                if let Some(msg) = self.context.worker_context_ref().get_msg() {
                    let message_style = Style::default().fg(Color::Yellow);
                    let text = Span::styled(msg, message_style);
                    Paragraph::new(text)
                        .wrap(Wrap { trim: true })
                        .render(footer_area, buf);
                } else if let Some(msg) = self.context.message_queue_ref().current_message() {
                    let text = Span::styled(msg.content.as_str(), msg.style);
                    Paragraph::new(text)
                        .wrap(Wrap { trim: true })
                        .render(footer_area, buf);
                } else {
                    TuiFooter::new(list, curr_tab.option_ref()).render(footer_area, buf);
                }
            }
        } else {
            match curr_tab.history_metadata_ref().get(curr_tab_cwd) {
                Some(PreviewDirState::Loading) => {
                    TuiDirListLoading::new().render(layout_rect[1], buf);
                }
                Some(PreviewDirState::Error { message }) => {
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
                Some(PreviewDirState::Loading) => {
                    TuiDirListLoading::new().render(layout_rect[2], buf);
                }
                Some(PreviewDirState::Error { message }) => {
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
                        if let Some(PreviewFileState::Success { data }) = preview_context
                            .previews_ref()
                            .get(&preview_area.file_preview_path)
                        {
                            TuiFilePreview::new(data).render(area, buf);
                        }
                    }
                }
            }
        } else {
            TuiMessage::new("Error loading directory", Style::default().fg(Color::Red))
                .render(layout_rect[2], buf);
        }

        let topbar_area = Self::header_area(&area);
        TuiTopBar::new(self.context, curr_tab_cwd).render(topbar_area, buf);

        // render tabs
        let tab_area = self.tab_area(&area, self.context.tab_context_ref().len());
        TuiTabBar::new(self.context.tab_context_ref()).render(tab_area, buf);
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
                Some(entry) => match preview_context.previews_ref().get(entry.file_path()) {
                    Some(PreviewFileState::Success { data }) if data.status.code() != Some(1) => {
                        &display_options.default_layout
                    }
                    Some(PreviewFileState::Loading) => &display_options.default_layout,
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
        if let Some(PreviewFileState::Success { data }) =
            preview_context.previews_ref().get(entry.file_path())
        {
            match data.status.code() {
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
