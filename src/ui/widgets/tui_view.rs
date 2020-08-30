use tui::buffer::Buffer;
use tui::layout::{Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Paragraph, Widget, Wrap};

use super::TuiFolderView;
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
    fn render(self, area: Rect, buf: &mut Buffer) {
        TuiFolderView::new(self.context).render(area, buf);
    }
}
