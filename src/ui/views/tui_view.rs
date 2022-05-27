use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use super::TuiFolderView;
use super::TuiVSplitView;
use crate::config::option::DisplayMode;
use crate::context::AppContext;

pub struct TuiView<'a> {
    pub context: &'a AppContext,
    pub show_bottom_status: bool,
}

impl<'a> TuiView<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Self {
            context,
            show_bottom_status: true,
        }
    }
}

impl<'a> Widget for TuiView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let config = self.context.config_ref();
        let display_options = config.display_options_ref();
        match display_options.mode() {
            DisplayMode::Default => {
                TuiFolderView::new(self.context).render(area, buf);
            }
            DisplayMode::VSplit => {
                TuiVSplitView::new(self.context).render(area, buf);
            }
        }
    }
}
