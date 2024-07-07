use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::types::option::display::DisplayMode;
use crate::types::state::AppState;
use crate::ui::views::{TuiFolderView, TuiHSplitView};

pub struct TuiView<'a> {
    pub app_state: &'a AppState,
    pub show_bottom_status: bool,
}

impl<'a> TuiView<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        Self {
            app_state,
            show_bottom_status: true,
        }
    }
}

impl<'a> Widget for TuiView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let display_options = &self.app_state.config.display_options;
        match display_options.mode {
            DisplayMode::Default => {
                TuiFolderView::new(self.app_state).render(area, buf);
            }
            DisplayMode::HSplit => {
                TuiHSplitView::new(self.app_state).render(area, buf);
            }
        }
    }
}
