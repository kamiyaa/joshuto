use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Widget};

use crate::THEME_T;

/// A simple bordered list-of-strings popup menu (used for bookmark selection).
pub struct TuiMenu<'a> {
    options: &'a [&'a str],
}

impl<'a> TuiMenu<'a> {
    /// Creates a menu widget listing `options`.
    pub fn new(options: &'a [&'a str]) -> Self {
        Self { options }
    }

    /// Returns the number of options in the menu.
    pub fn len(&self) -> usize {
        self.options.len()
    }
}

impl Widget for TuiMenu<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default().fg(Color::Reset).bg(Color::Reset);

        Block::default()
            .style(style)
            .borders(Borders::TOP)
            .border_style(THEME_T.border.as_style())
            .render(area, buf);

        let text_iter = self.options.iter().chain(&[" "]);
        let area_x = area.x + 1;

        for (y, text) in (area.y + 1..area.y + area.height).zip(text_iter) {
            buf.set_string(area_x, y, text, style);
        }
    }
}
