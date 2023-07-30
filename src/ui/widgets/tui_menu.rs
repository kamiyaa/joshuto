use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Widget};

pub struct TuiMenu<'a> {
    options: &'a [&'a str],
}

impl<'a> TuiMenu<'a> {
    pub fn new(options: &'a [&'a str]) -> Self {
        Self { options }
    }

    pub fn len(&self) -> usize {
        self.options.len()
    }
}

impl<'a> Widget for TuiMenu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default().fg(Color::Reset).bg(Color::Reset);

        Block::default()
            .style(style)
            .borders(Borders::TOP)
            .render(area, buf);

        let text_iter = self.options.iter().chain(&[" "]);
        let area_x = area.x + 1;

        for (y, text) in (area.y + 1..area.y + area.height).zip(text_iter) {
            buf.set_string(area_x, y, text, style);
        }
    }
}
