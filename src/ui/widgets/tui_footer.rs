use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget};

use crate::fs::{FileType, JoshutoDirList};
use crate::util::format;
use crate::util::unix;

pub struct TuiFooter<'a> {
    dirlist: &'a JoshutoDirList,
}

impl<'a> TuiFooter<'a> {
    pub fn new(dirlist: &'a JoshutoDirList) -> Self {
        Self { dirlist }
    }
}

impl<'a> Widget for TuiFooter<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use std::os::unix::fs::PermissionsExt;
        match self.dirlist.index {
            Some(i) if i < self.dirlist.len() => {
                let entry = &self.dirlist.contents[i];

                let mode_style = Style::default().fg(Color::Cyan);
                let mode_str = unix::mode_to_string(entry.metadata.permissions_ref().mode());

                let mtime_str = format::mtime_to_string(entry.metadata.modified());
                let size_str = format::file_size_to_string(entry.metadata.len());

                #[cfg(unix)]
                let mimetype = match entry.metadata.mimetype.as_ref() {
                    Some(s) => s.as_str(),
                    None => "",
                };

                let mut text = vec![
                    Span::styled(mode_str, mode_style),
                    Span::raw("  "),
                    Span::raw(format!("{}/{}", i + 1, self.dirlist.len())),
                    Span::raw("  "),
                    Span::raw(mtime_str),
                    Span::raw("  "),
                    Span::raw(size_str),
                    #[cfg(unix)]
                    Span::raw("  "),
                    #[cfg(unix)]
                    Span::raw(mimetype),
                ];

                if let FileType::Symlink(s) = entry.metadata.file_type() {
                    text.push(Span::styled(" -> ", mode_style));
                    text.push(Span::styled(s, mode_style));
                }

                Paragraph::new(Spans::from(text)).render(area, buf);
            }
            _ => {}
        }
    }
}
