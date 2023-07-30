use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::context::AppContext;
use crate::ui::widgets::{TuiTopBar, TuiWorker};

pub struct TuiWorkerView<'a> {
    context: &'a AppContext,
}

impl<'a> TuiWorkerView<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Self { context }
    }
}

impl<'a> Widget for TuiWorkerView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }

        let rect = Rect { height: 1, ..area };
        let curr_tab = self.context.tab_context_ref().curr_tab_ref();
        TuiTopBar::new(self.context, curr_tab.cwd()).render(rect, buf);

        let rect = Rect {
            x: 0,
            y: 1,
            width: area.width,
            height: area.height - 1,
        };
        TuiWorker::new(self.context.worker_context_ref()).render(rect, buf);
    }
}
