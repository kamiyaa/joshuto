use ansi_to_tui::ansi_to_text;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::text::{Span, Text};
use tui::widgets::Widget;

use crate::preview::preview_file::FilePreview;

pub struct TuiFilePreview<'a> {
    preview: &'a FilePreview,
}

impl<'a> TuiFilePreview<'a> {
    pub fn new(preview: &'a FilePreview) -> Self {
        Self { preview }
    }
}

impl<'a> Widget for TuiFilePreview<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vec = self.preview.output.as_str().as_bytes().to_vec();
        let res = ansi_to_text(vec);
        match res {
            Ok(text) => {
                for (line, y) in text
                    .lines
                    .iter()
                    .skip(self.preview.index)
                    .zip(area.y..area.y + area.height)
                {
                    buf.set_spans(area.x, y, line, area.width);
                }
            }
            Err(e) => {
                let span = Span::raw(format!("Failed to parse ansi colors: {}", e));
                buf.set_span(area.x, area.y, &span, area.width);

                let vec = self.preview.output.as_str().as_bytes().to_vec();
                for (line, y) in vec
                    .iter()
                    .skip(self.preview.index)
                    .zip(area.y+1..area.y + area.height)
                {
                    let span = Span::raw(line.to_string());
                    buf.set_span(area.x, y, &span, area.width);
                }
            }
        }
    }
}
