use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget, Wrap};

use uuid::Uuid;

pub struct TuiTabBar<'a> {
    tabs: &'a [Uuid],
    index: usize,
}

impl<'a> TuiTabBar<'a> {
    pub fn new(tabs: &'a [Uuid], index: usize) -> Self {
        Self { tabs, index }
    }
}

impl<'a> Widget for TuiTabBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let regular_style = Style::default().fg(Color::White);
        let selected_style = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::REVERSED);

        let mut spans_vec = vec![];
        for i in 0..self.tabs.len() {
            if i == self.index {
                spans_vec.push(Span::styled(
                    self.tabs[i].to_string()[..4].to_string(),
                    selected_style,
                ));
                spans_vec.push(Span::styled(" ", regular_style));
            } else {
                spans_vec.push(Span::styled(
                    self.tabs[i].to_string()[..4].to_string(),
                    regular_style,
                ));
                spans_vec.push(Span::styled(" ", regular_style));
            }
        }

        Paragraph::new(Spans::from(spans_vec))
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
