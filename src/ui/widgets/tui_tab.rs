use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::{Paragraph, Text, Widget};

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
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let selected = Style::default().modifier(Modifier::REVERSED);

        let text = [
            Text::styled(format!("{}: {}", self.curr + 1, self.name), selected),
            Text::raw(format!("/{}", self.len)),
        ];

        Paragraph::new(text.iter()).wrap(true).draw(area, buf);
    }
}
