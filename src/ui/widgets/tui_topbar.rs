use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use crate::types::state::AppState;
use crate::{HOSTNAME, USERNAME};

use super::tui_tab_bar::TuiTabBar;

pub struct TuiTopBar<'a> {
    pub app_state: &'a AppState,
}

impl<'a> TuiTopBar<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        Self { app_state }
    }
}

impl Widget for TuiTopBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let name_width = USERNAME.as_str().len() + HOSTNAME.as_str().len() + 2;

        let username_style = if USERNAME.as_str() == "root" {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        };

        let top_bar_spans = vec![
            Span::styled(USERNAME.as_str(), username_style),
            Span::styled("@", username_style),
            Span::styled(HOSTNAME.as_str(), username_style),
            Span::styled(" ", username_style),
        ];
        Paragraph::new(Line::from(top_bar_spans)).render(area, buf);

        let available_tab_width = area.width as usize - name_width;
        let tab_area = Rect {
            x: name_width as u16,
            width: available_tab_width as u16,
            ..area
        };

        let tab_bar = TuiTabBar::new(
            &self.app_state.config,
            self.app_state.state.tab_state_ref().tab_refs_in_order(),
            self.app_state.state.tab_state_ref().index,
        );
        tab_bar.render(tab_area, buf);
    }
}
