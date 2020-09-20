use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget};

use crate::fs::{FileType, JoshutoDirList};
use crate::util::format;

pub struct TuiFooter<'a> {
    list: &'a JoshutoDirList,
}

impl<'a> TuiFooter<'a> {
    pub fn new(list: &'a JoshutoDirList) -> Self {
        Self { list }
    }
}

impl<'a> Widget for TuiFooter<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use std::os::unix::fs::PermissionsExt;
        match self.list.index {
            Some(i) if i < self.list.contents.len() => {
                let entry = &self.list.contents[i];
                let mode = entry.metadata.permissions.mode();
                let mode = format::mode_to_string(mode);

                let mode_style = Style::default().fg(Color::Cyan);

                let mtime = entry.metadata.modified;
                let mtime = format::mtime_to_string(mtime);

                let size = entry.metadata.len;
                let size = format::file_size_to_string(size);

                #[cfg(unix)]
                let mimetype = match entry.metadata.mimetype.as_ref() {
                    Some(s) => s,
                    None => "",
                };

                let mut text = vec![
                    Span::styled(mode, mode_style),
                    Span::raw("  "),
                    Span::raw(format!("{}/{}", i + 1, self.list.contents.len())),
                    Span::raw("  "),
                    Span::raw(mtime),
                    Span::raw("  "),
                    Span::raw(size),
                    #[cfg(unix)]
                    Span::raw("  "),
                    #[cfg(unix)]
                    Span::raw(mimetype),
                ];

                if let FileType::Symlink(s) = &entry.metadata.file_type {
                    text.push(Span::styled(" -> ", mode_style));
                    text.push(Span::styled(s, mode_style));
                }

                Paragraph::new(Spans::from(text)).render(area, buf);
            }
            _ => {}
        }
    }
}
