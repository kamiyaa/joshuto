use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::fs::{JoshutoDirEntry, JoshutoDirList};
use crate::ui::widgets::trim_file_label;
use crate::util::style;

pub struct TuiDirListLoading;

impl TuiDirListLoading {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for TuiDirListLoading {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 1 {
            return;
        }
        let x = area.left();
        let y = area.top();

        let style = Style::default().fg(Color::Yellow);
        buf.set_stringn(x, y, "loading...", area.width as usize, style);
    }
}
