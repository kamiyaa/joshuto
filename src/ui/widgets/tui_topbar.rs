use std::path::Path;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Paragraph, Text, Widget};

use crate::{HOSTNAME, USERNAME};

pub struct TuiTopBar<'a> {
    path: &'a Path,
}

impl<'a> TuiTopBar<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }
}

impl<'a> Widget for TuiTopBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let username_style = Style::default()
            .fg(Color::LightGreen)
            .modifier(Modifier::BOLD);

        let path_style = Style::default()
            .fg(Color::LightBlue)
            .modifier(Modifier::BOLD);

        let curr_path_str = self.path.to_string_lossy();

        let text = [
            Text::styled(format!("{}@{} ", *USERNAME, *HOSTNAME), username_style),
            Text::styled(curr_path_str, path_style),
        ];

        Paragraph::new(text.iter()).wrap(true).render(area, buf);
    }
}
