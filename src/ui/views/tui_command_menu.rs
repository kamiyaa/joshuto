use std::iter::Iterator;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Clear, Widget};

use crate::traits::ToString;
use crate::types::keybind::KeyMapping;
use crate::types::state::AppState;
use crate::ui::views::TuiView;
use crate::ui::widgets::TuiMenu;

const BORDER_HEIGHT: usize = 1;
const BOTTOM_MARGIN: usize = 1;

pub struct TuiCommandMenu<'a> {
    app_state: &'a AppState,
    keymap: &'a KeyMapping,
}

impl<'a> TuiCommandMenu<'a> {
    pub fn new(app_state: &'a AppState, keymap: &'a KeyMapping) -> Self {
        Self { app_state, keymap }
    }
}

impl Widget for TuiCommandMenu<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        TuiView::new(self.app_state).render(area, buf);

        // draw menu
        let mut display_vec: Vec<String> = self
            .keymap
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
