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

/// The top status bar: username@hostname (if enabled) followed by the tab bar.
pub struct TuiTopBar<'a> {
    pub app_state: &'a AppState,
}

impl<'a> TuiTopBar<'a> {
    /// Creates the top bar widget for the current `app_state`.
    pub fn new(app_state: &'a AppState) -> Self {
        Self { app_state }
    }
}

impl Widget for TuiTopBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let show_hostname = self.app_state.config.display_options.show_hostname;

        let name_width = if show_hostname {
            USERNAME.as_str().len() + HOSTNAME.as_str().len() + 2
        } else {
            0
        };

        let mut top_bar_spans = Vec::new();
        if show_hostname {
            let username_style = if USERNAME.as_str() == "root" {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
            };

            top_bar_spans.push(Span::styled(USERNAME.as_str(), username_style));
            top_bar_spans.push(Span::styled("@", username_style));
            top_bar_spans.push(Span::styled(HOSTNAME.as_str(), username_style));
            top_bar_spans.push(Span::styled(" ", username_style));
        }

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
