use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use super::TuiFolderView;
use crate::context::JoshutoContext;

pub struct TuiView<'a> {
    pub context: &'a JoshutoContext,
    pub show_bottom_status: bool,
}

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
