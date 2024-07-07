use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use tab_list_builder::factor_tab_bar_spans;

use crate::types::state::AppState;
use crate::ui::tab_list_builder;
use crate::THEME_T;
use crate::{HOSTNAME, USERNAME};

pub struct TuiTopBar<'a> {
    pub app_state: &'a AppState,
}

impl<'a> TuiTopBar<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        Self { app_state }
    }
}

impl<'a> Widget for TuiTopBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let name_width = USERNAME.as_str().len() + HOSTNAME.as_str().len() + 2;

        let username_style = if USERNAME.as_str() == "root" {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        };

        let mut top_bar_spans = vec![
            Span::styled(USERNAME.as_str(), username_style),
            Span::styled("@", username_style),
            Span::styled(HOSTNAME.as_str(), username_style),
            Span::styled(" ", username_style),
        ];

        let available_tab_width = area.width as usize - name_width;
        let mut paths = Vec::new();
        let tabs = self.app_state.state.tab_state_ref().tab_refs_in_order();
        for tab in tabs {
            paths.push(tab.get_cwd());
        }
        let tab_bar_spans = factor_tab_bar_spans(
            available_tab_width,
            &paths,
            self.app_state.state.tab_state_ref().index,
            &THEME_T.tabs,
        );
        top_bar_spans.extend(tab_bar_spans);
        Paragraph::new(Line::from(top_bar_spans)).render(area, buf);
    }
}
