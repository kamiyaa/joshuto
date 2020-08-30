use tui::buffer::Buffer;
use tui::layout::{Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Paragraph, Widget, Wrap};

use super::{TuiDirList, TuiDirListDetailed, TuiFooter, TuiTabBar, TuiTopBar};
use crate::context::JoshutoContext;

const TAB_VIEW_WIDTH: u16 = 15;

pub struct TuiWorkerView<'a> {
    pub context: &'a JoshutoContext,
    pub show_bottom_status: bool,
}

use super::super::{DEFAULT_LAYOUT, NO_PREVIEW_LAYOUT};

impl<'a> TuiWorkerView<'a> {
    pub fn new(context: &'a JoshutoContext) -> Self {
        Self {
            context,
            show_bottom_status: true,
        }
    }
}

impl<'a> Widget for TuiWorkerView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let f_size = area;

        let curr_tab = self.context.tab_context_ref().curr_tab_ref();
        let layout_rect = Layout::default().direction(Direction::Horizontal).margin(1);

        if self.context.tab_context_ref().len() > 1 {
            let topbar_width = if f_size.width > TAB_VIEW_WIDTH {
                f_size.width - TAB_VIEW_WIDTH
            } else {
                0
            };

            let rect = Rect {
                x: 0,
                y: 0,
                width: topbar_width,
                height: 1,
            };
            TuiTopBar::new(curr_tab.pwd()).render(rect, buf);

            let rect = Rect {
                x: topbar_width,
                y: 0,
                width: TAB_VIEW_WIDTH,
                height: 1,
            };
            let name = if let Some(ostr) = curr_tab.pwd().file_name() {
                ostr.to_str().unwrap_or("")
            } else {
                ""
            };
            TuiTabBar::new(
                name,
                self.context.tab_context_ref().get_index(),
                self.context.tab_context_ref().len(),
            )
            .render(rect, buf);
        } else {
            let topbar_width = f_size.width;

            let rect = Rect {
                x: 0,
                y: 0,
                width: topbar_width,
                height: 1,
            };
            TuiTopBar::new(curr_tab.pwd()).render(rect, buf);
        }
    }
}
