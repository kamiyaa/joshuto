use std::path::Component;
use std::path::Path;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use unicode_width::UnicodeWidthStr;

use crate::context::AppContext;
use crate::{HOME_DIR, HOSTNAME, USERNAME};

pub struct TuiTopBar<'a> {
    pub context: &'a AppContext,
    path: &'a Path,
}

impl<'a> TuiTopBar<'a> {
    pub fn new(context: &'a AppContext, path: &'a Path) -> Self {
        Self { context, path }
    }
}

impl<'a> Widget for TuiTopBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let path_style = Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD);

        let mut ellipses = None;
        let mut curr_path_str = self.path.to_string_lossy().into_owned();

        let tab_width = self.context.tab_context_ref().tab_area_width();
        let name_width = USERNAME.as_str().len() + HOSTNAME.as_str().len() + 2;

        if tab_width + name_width > area.width as usize {
            curr_path_str = "".to_owned();
        } else if curr_path_str.width() > area.width as usize - tab_width - name_width {
            if let Some(s) = self.path.file_name() {
                let mut short_path = String::new();
                let mut components: Vec<Component> = self.path.components().collect();
                components.pop();

                for component in components {
                    match component {
                        Component::RootDir => short_path.push('/'),
                        Component::Normal(s) => {
                            let ch = s.to_string_lossy().chars().next().unwrap();
                            short_path.push(ch);
                            short_path.push('/');
                        }
                        _ => {}
                    }
                }
                ellipses = Some(Span::styled(short_path, path_style));
                curr_path_str = s.to_string_lossy().into_owned();
            }
        }
        if self
            .context
            .config_ref()
            .display_options_ref()
            .tilde_in_titlebar()
        {
            if let Some(home_dir) = HOME_DIR.as_ref() {
                let home_dir_str = home_dir.to_string_lossy().into_owned();
                curr_path_str = curr_path_str.replace(&home_dir_str, "~");
            }
        }

        let username_style = if USERNAME.as_str() == "root" {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        };

        let mut text = vec![
            Span::styled(USERNAME.as_str(), username_style),
            Span::styled("@", username_style),
            Span::styled(HOSTNAME.as_str(), username_style),
            Span::styled(" ", username_style),
        ];

        if let Some(s) = ellipses {
            text.push(s);
        }

        text.extend([Span::styled(curr_path_str, path_style)]);

        Paragraph::new(Line::from(text)).render(area, buf);
    }
}
