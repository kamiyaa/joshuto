use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::types::state::AppState;
use crate::ui::widgets::{TuiIoTasks, TuiTopBar};

pub struct TuiWorkerView<'a> {
    app_state: &'a AppState,
}

impl<'a> TuiWorkerView<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        Self { app_state }
    }
}

impl<'a> Widget for TuiWorkerView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }

        let rect = Rect { height: 1, ..area };
        TuiTopBar::new(self.app_state).render(rect, buf);

        let rect = Rect {
            x: 0,
            y: 1,
            width: area.width,
            height: area.height - 1,
        };
        TuiIoTasks::new(self.app_state.state.worker_state_ref()).render(rect, buf);
    }
}
