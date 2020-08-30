use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget, Wrap};

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
        let selected = Style::default()
            .add_modifier(Modifier::REVERSED);

        let text = Spans::from(vec![
            Span::styled(format!("{}: {}", self.curr + 1, self.name), selected),
            Span::raw(format!("/{}", self.len)),
        ]);

        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
