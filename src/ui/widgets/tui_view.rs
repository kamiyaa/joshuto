use tui::buffer::Buffer;
use tui::layout::{Direction, Layout, Rect};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use super::{TuiDirList, TuiDirListDetailed, TuiFooter, TuiTopBar};
use crate::context::JoshutoContext;

pub struct TuiView<'a> {
    pub context: &'a JoshutoContext,
}

use super::super::{DEFAULT_LAYOUT, NO_PREVIEW_LAYOUT};

impl<'a> TuiView<'a> {
    pub fn new(context: &'a JoshutoContext) -> Self {
        Self { context }
    }
}

impl<'a> Widget for TuiView<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let curr_tab = self.context.curr_tab_ref();

        let curr_list = curr_tab.curr_list_ref();
        let parent_list = curr_tab.parent_list_ref();
        let child_list = curr_tab.child_list_ref();

        let f_size = area;

        let constraints = match child_list {
            Some(_) => DEFAULT_LAYOUT,
            None => NO_PREVIEW_LAYOUT,
        };
        let layout_rect = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(constraints.as_ref())
            .split(f_size);

        {
            let rect = Rect {
                x: 0,
                y: 0,
                width: f_size.width,
                height: 1,
            };

            TuiTopBar::new(curr_tab.curr_path.as_path()).draw(rect, buf);
        }

        if let Some(curr_list) = parent_list.as_ref() {
            TuiDirList::new(&curr_list).draw(layout_rect[0], buf);
        };

        if let Some(curr_list) = curr_list.as_ref() {
            TuiDirListDetailed::new(&curr_list).draw(layout_rect[1], buf);

            if let Some(entry) = curr_list.get_curr_ref() {
                let rect = Rect {
                    x: 0,
                    y: f_size.height - 1,
                    width: f_size.width,
                    height: 1,
                };
                TuiFooter::new(entry).draw(rect, buf);
            }
        };

        if let Some(curr_list) = child_list.as_ref() {
            TuiDirList::new(&curr_list).draw(layout_rect[2], buf);
        };
    }
}
