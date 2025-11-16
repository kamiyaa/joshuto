use crate::{THEME_T, TIMEZONE_STR};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Widget};

pub struct TuiMenu<'a> {
    options: &'a [&'a str],
    highlighted_index: Option<usize>,
}

impl<'a> TuiMenu<'a> {
    pub fn new(options: &'a [&'a str]) -> Self {
        Self {
            options,
            highlighted_index: None,
        }
    }

    pub fn highlighted_index(mut self, index: Option<usize>) -> Self {
        self.highlighted_index = index;
        self
    }

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
            .render(area, buf);

        let text_iter = self.options.iter().chain(&[" "]);
        let area_x = area.x + 1;

        for (y, (index, text)) in (area.y + 1..area.y + area.height).zip(text_iter.enumerate()) {
            let item_style = if Some(index) == self.highlighted_index {
                Style::default()
                    .fg(THEME_T.menu.fg)
                    .bg(THEME_T.menu.bg)
                    .add_modifier(THEME_T.selection.modifier)
            } else {
                style
            };

            buf.set_string(area_x, y, text, item_style);
        }
    }
}
