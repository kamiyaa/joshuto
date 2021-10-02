use std::iter::Iterator;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{Clear, Widget};

use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::ui::views::TuiView;
use crate::ui::widgets::TuiMenu;
use crate::util::to_string::ToString;

const BORDER_HEIGHT: usize = 1;
const BOTTOM_MARGIN: usize = 1;

pub struct TuiCommandMenu<'a> {
    context: &'a AppContext,
    keymap: &'a AppKeyMapping,
}

impl<'a> TuiCommandMenu<'a> {
    pub fn new(context: &'a AppContext, keymap: &'a AppKeyMapping) -> Self {
        Self { context, keymap }
    }
}

impl<'a> Widget for TuiCommandMenu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        TuiView::new(self.context).render(area, buf);

        // draw menu
        let mut display_vec: Vec<String> = self
            .keymap
            .as_ref()
            .iter()
            .map(|(k, v)| format!("  {}        {}", k.to_string(), v))
            .collect();
        display_vec.sort();
        let display_str: Vec<&str> = display_vec.iter().map(|v| v.as_str()).collect();
        let display_str_len = display_str.len();

        let y = if (area.height as usize) < display_str_len + BORDER_HEIGHT + BOTTOM_MARGIN {
            0
        } else {
            area.height - (BORDER_HEIGHT + BOTTOM_MARGIN) as u16 - display_str_len as u16
        };

        let menu_height = if display_str_len + BORDER_HEIGHT > area.height as usize {
            area.height
        } else {
            (display_str_len + BORDER_HEIGHT) as u16
        };

        let menu_rect = Rect {
            x: 0,
            y,
            width: area.width,
            height: menu_height,
        };

        Clear.render(menu_rect, buf);
        TuiMenu::new(&display_str).render(menu_rect, buf);
    }
}
