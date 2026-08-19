use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::types::option::display::DisplayMode;
use crate::types::state::AppState;
use crate::ui::views::{tui_minimal_view::TuiMinimalView, TuiFolderView, TuiHSplitView};

/// The main file-manager screen, delegating to the configured [`DisplayMode`]'s view.
pub struct TuiView<'a> {
    pub app_state: &'a AppState,
    pub show_bottom_status: bool,
}

impl<'a> TuiView<'a> {
    /// Creates the main view for `app_state`, with the bottom status line shown by default.
    pub fn new(app_state: &'a AppState) -> Self {
        Self {
            app_state,
            show_bottom_status: true,
        }
    }
}

impl Widget for TuiView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let display_options = &self.app_state.config.display_options;
        match display_options.mode {
            DisplayMode::Default => {
                TuiFolderView::new(self.app_state).render(area, buf);
            }
            DisplayMode::Minimal => {
                TuiMinimalView::new(self.app_state).render(area, buf);
            }
            DisplayMode::HSplit => {
                TuiHSplitView::new(self.app_state).render(area, buf);
            }
        }
    }
}
