use std::ffi::OsStr;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget, Wrap};

use crate::context::TabContext;

pub struct TuiTabBar<'a> {
    context: &'a TabContext,
}

impl<'a> TuiTabBar<'a> {
    pub fn new(context: &'a TabContext) -> Self {
        Self { context }
    }
}

impl<'a> Widget for TuiTabBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let regular_style = Style::default().fg(Color::White);
        let selected_style = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::REVERSED);

        let index = self.context.index;
        let tab_order = self.context.tab_order.as_slice();

        let mut spans_vec = vec![];
        for i in 0..tab_order.len() {
            let curr_style = if i == index {
                selected_style
            } else {
                regular_style
            };
            let tab_id = &tab_order[i];
            if let Some(curr_tab) = self.context.tab_ref(tab_id) {
                let preview_text: String = curr_tab
                    .cwd()
                    .file_name()
                    .unwrap_or(OsStr::new("/"))
                    .to_string_lossy()
                    .chars()
                    .take(4)
                    .collect();

                spans_vec.push(Span::styled(
                    format!("{}: {}", i + 1, preview_text),
                    curr_style,
                ));
                spans_vec.push(Span::styled(" ", regular_style));
            }
        }

        Paragraph::new(Spans::from(spans_vec))
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
