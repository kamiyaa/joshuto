use std::ffi::OsStr;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget, Wrap};

use crate::context::TabContext;
use crate::THEME_T;

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
        let regular_style = THEME_T.tabs.inactive.as_style();
        let selected_style = THEME_T.tabs.active.as_style();

        let index = self.context.index;
        let tab_order = self.context.tab_order.as_slice();

        let mut spans_vec = vec![];
        for (i, tab_id) in tab_order.iter().enumerate() {
            let curr_style = if i == index {
                selected_style
            } else {
                regular_style
            };
            if let Some(curr_tab) = self.context.tab_ref(tab_id) {
                let preview_text: String = curr_tab
                    .cwd()
                    .file_name()
                    .unwrap_or_else(|| OsStr::new("/"))
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

        Paragraph::new(Line::from(spans_vec))
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
