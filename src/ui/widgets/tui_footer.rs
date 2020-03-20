use std::fs;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Paragraph, Text, Widget};

use crate::fs::JoshutoDirEntry;
use crate::util::format;

pub struct TuiFooter<'a> {
    entry: &'a JoshutoDirEntry,
}

impl<'a> TuiFooter<'a> {
    pub fn new(entry: &'a JoshutoDirEntry) -> Self {
        Self { entry }
    }
}

impl<'a> Widget for TuiFooter<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        use std::os::unix::fs::PermissionsExt;

        let mode = self.entry.metadata.permissions.mode();
        let mode = format::mode_to_string(mode);

        let mode_style = Style::default().fg(Color::Cyan);

        let mtime = self.entry.metadata.modified;
        let mtime = format::mtime_to_string(mtime);

        let size = self.entry.metadata.len;
        let size = format::file_size_to_string(size as f64);

        #[cfg(unix)]
        let mimetype = match self.entry.metadata.mimetype.as_ref() {
            Some(s) => s,
            None => "",
        };

        let mut text = vec![
            Text::styled(mode, mode_style),
            Text::raw("  "),
            Text::raw(mtime),
            Text::raw("  "),
            Text::raw(size),
            #[cfg(unix)]
            Text::raw("  "),
            #[cfg(unix)]
            Text::raw(mimetype),
        ];

        if self.entry.metadata.file_type.is_symlink() {
            if let Ok(path) = fs::read_link(self.entry.file_path()) {
                text.push(Text::styled(" -> ", mode_style));
                if let Some(s) = path.to_str() {
                    text.push(Text::styled(s.to_string(), mode_style));
                }
            }
        }

        Paragraph::new(text.iter()).wrap(true).draw(area, buf);
    }
}
