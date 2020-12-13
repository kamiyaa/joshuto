use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget, Wrap};

use unicode_width::UnicodeWidthStr;

pub struct TuiTabBar<'a> {
    name: &'a str,
    curr: usize,
    len: usize,
}

impl<'a> TuiTabBar<'a> {
    pub fn new(name: &'a str, curr: usize, len: usize) -> Self {
        Self { name, curr, len }
    }
}

impl<'a> Widget for TuiTabBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let selected = Style::default().add_modifier(Modifier::REVERSED);

        let str1 = format!("{}/{}", self.curr + 1, self.len);
        let str2 = {
            let space_avail = if str1.width() >= area.width as usize {
                0
            } else {
                area.width as usize - str1.len()
            };
            if space_avail >= self.name.width() {
                self.name
            } else {
                "…"
            }
        };
        let text = Spans::from(vec![
            Span::styled(str1, selected),
            Span::styled(": ", selected),
            Span::styled(str2, selected),
        ]);

        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
