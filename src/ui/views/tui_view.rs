use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use super::TuiFolderView;
use crate::context::AppContext;
use crate::ui::RenderResult;

pub struct TuiView<'a> {
    pub context: &'a AppContext,
    pub show_bottom_status: bool,
    pub render_result: &'a mut RenderResult,
}

impl<'a> TuiView<'a> {
    pub fn new(context: &'a AppContext, render_result: &'a mut RenderResult) -> Self {
        Self {
            context,
            show_bottom_status: true,
            render_result,
        }
    }
}

impl<'a> Widget for TuiView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        TuiFolderView::new(self.context, self.render_result).render(area, buf);
    }
}
