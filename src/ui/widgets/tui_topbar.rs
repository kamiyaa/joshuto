use std::path::Component;
use std::path::Path;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Line};
use tui::widgets::{Paragraph, Widget};

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

        if curr_path_str.width() > area.width as usize {
            if let Some(s) = self.path.file_name() {
                let mut short_path = String::new();
                for component in self.path.components() {
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

        let text = match ellipses {
            Some(s) => Line::from(vec![
                Span::styled(USERNAME.as_str(), username_style),
                Span::styled("@", username_style),
                Span::styled(HOSTNAME.as_str(), username_style),
                Span::styled(" ", username_style),
                s,
                Span::styled(curr_path_str, path_style),
            ]),
            None => Line::from(vec![
                Span::styled(USERNAME.as_str(), username_style),
                Span::styled("@", username_style),
                Span::styled(HOSTNAME.as_str(), username_style),
                Span::styled(" ", username_style),
                Span::styled(curr_path_str, path_style),
            ]),
        };

        Paragraph::new(text).render(area, buf);
    }
}
