use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::style::{Color, Style};
use ratatui::symbols::line::{HORIZONTAL_DOWN, HORIZONTAL_UP};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use ratatui_image::Image;

use crate::fs::FileType;
use crate::preview::preview_dir::PreviewDirState;
use crate::preview::preview_file::PreviewFileState;
use crate::types::state::{AppState, PreviewState, TabState};
use crate::ui;
use crate::ui::widgets::{
    TuiDirList, TuiDirListDetailed, TuiDirListLoading, TuiFilePreview, TuiFooter, TuiMessage,
    TuiTopBar,
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

impl Widget for TuiFolderViewBorders<'_> {
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
    pub app_state: &'a AppState,
    pub show_bottom_status: bool,
}

impl<'a> TuiFolderView<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        Self {
            app_state,
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
}

impl Widget for TuiFolderView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let display_options = &self.app_state.config.display_options;

        let preview_state = self.app_state.state.preview_state_ref();
        let curr_tab = self.app_state.state.tab_state_ref().curr_tab_ref();
        let curr_tab_cwd = curr_tab.get_cwd();

        let curr_list = curr_tab.curr_list_ref();
        let child_list = curr_tab.child_list_ref();

        let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());

        let constraints = get_constraints(self.app_state);

        let layout_rect = if display_options.show_borders {
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
                    TuiDirList::new(&self.app_state.config, list, true).render(layout_rect[0], buf);
                }
            }
        }

        // render current view
        if let Some(list) = curr_list.as_ref() {
            TuiDirListDetailed::new(
                &self.app_state.config,
                list,
                display_options,
                curr_tab.option_ref(),
                true,
            )
            .render(layout_rect[1], buf);

            let footer_area = Self::footer_area(&area);
            if self.show_bottom_status {
                /* draw the bottom status bar */
                if let Some(msg) = self.app_state.state.worker_state_ref().get_msg() {
                    let message_style = Style::default().fg(Color::Yellow);
                    let text = Span::styled(msg, message_style);
                    Paragraph::new(text)
                        .wrap(Wrap { trim: true })
                        .render(footer_area, buf);
                } else if let Some(msg) = self.app_state.state.message_queue_ref().current_message()
                {
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
            TuiDirList::new(&self.app_state.config, list, true).render(layout_rect[2], buf);
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
                    let image_offset = match preview_state.image_preview_ref(entry.file_path()) {
                        Some(protocol) => {
                            let area = layout_rect[2];
                            Image::new(protocol).render(area, buf);
                            protocol.rect().height
                        }
                        _ => 0,
                    };

                    if let Some(PreviewFileState::Success(data)) =
                        preview_state.previews_ref().get(entry.file_path())
                    {
                        let preview_area = calculate_preview(
                            self.app_state.state.tab_state_ref(),
                            self.app_state.state.preview_state_ref(),
                            layout_rect[2],
                        );
                        if let Some(preview_area) = preview_area {
                            let area = Rect {
                                x: preview_area.preview_area.x,
                                y: preview_area.preview_area.y + image_offset,
                                width: preview_area.preview_area.width,
                                height: preview_area
                                    .preview_area
                                    .height
                                    .saturating_sub(image_offset),
                            };
                            TuiFilePreview::new(data).render(area, buf);
                        }
                    };
                }
            }
        } else {
            TuiMessage::new("Error loading directory", Style::default().fg(Color::Red))
                .render(layout_rect[2], buf);
        }

        let topbar_area = Self::header_area(&area);
        TuiTopBar::new(self.app_state).render(topbar_area, buf);
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
        if let Some(cell) = buf.cell_mut(Position::new(self.left, self.top)) {
            *cell = Cell::new(HORIZONTAL_DOWN);
        }
        if let Some(cell) = buf.cell_mut(Position::new(self.left, self.bottom)) {
            *cell = Cell::new(HORIZONTAL_UP);
        }
    }
    fn render_right(&self, buf: &mut Buffer) {
        if let Some(cell) = buf.cell_mut(Position::new(self.right, self.top)) {
            *cell = Cell::new(HORIZONTAL_DOWN);
        }
        if let Some(cell) = buf.cell_mut(Position::new(self.right, self.bottom)) {
            *cell = Cell::new(HORIZONTAL_UP);
        }
    }
}

pub fn get_constraints(app_state: &AppState) -> &[Constraint; 3] {
    let display_options = &app_state.config.display_options;
    if app_state.state.tab_state_ref().len() == 0 {
        return &display_options.default_layout;
    }

    let preview_state = app_state.state.preview_state_ref();
    let curr_tab = app_state.state.tab_state_ref().curr_tab_ref();

    let curr_list = curr_tab.curr_list_ref();
    let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());

    let child_list = curr_tab.child_list_ref();

    if !display_options.collapse_preview {
        &display_options.default_layout
    } else {
        match child_list {
            Some(_) => &display_options.default_layout,
            None => match curr_entry {
                None => &display_options.no_preview_layout,
                Some(entry) if entry.metadata.file_type() == FileType::Directory => {
                    &display_options.default_layout
                }
                Some(entry) => match preview_state.previews_ref().get(entry.file_path()) {
                    None => &display_options.no_preview_layout,
                    _ => &display_options.default_layout,
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

pub fn calculate_preview(
    tab_state: &TabState,
    preview_state: &PreviewState,
    rect: Rect,
) -> Option<PreviewArea> {
    let curr_tab = tab_state.curr_tab_ref();

    let child_list = curr_tab.child_list_ref();

    let curr_list = curr_tab.curr_list_ref();
    let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());

    if child_list.as_ref().is_some() {
        None
    } else if let Some(entry) = curr_entry {
        match preview_state.previews_ref().get(entry.file_path()) {
            None | Some(PreviewFileState::Loading) | Some(PreviewFileState::Error(_)) => None,
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
}
