use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Span;
use ratatui::widgets::Widget;

use crate::preview::preview_file::FilePreview;

pub struct TuiFilePreview<'a> {
    preview: &'a FilePreview,
}

impl<'a> TuiFilePreview<'a> {
    pub fn new(preview: &'a FilePreview) -> Self {
        Self { preview }
    }

    #[cfg(not(feature = "syntax_highlight"))]
    fn render_text_preview(&self, area: Rect, buf: &mut Buffer, s: &str) {
        let vec: Vec<&str> = s.split('\n').collect();
        for (line, y) in vec
            .iter()
            .skip(self.preview.index)
            .zip(area.y..area.y + area.height)
        {
            let span = Span::raw(line.to_string());
            buf.set_span(area.x, y, &span, area.width);
        }
    }

    #[cfg(feature = "syntax_highlight")]
    fn render_text_preview(&self, area: Rect, buf: &mut Buffer, s: &str) {
        use ansi_to_tui::IntoText;
        let vec = s.as_bytes().to_vec();
        let res = vec.into_text();

        match res {
            Ok(text) => {
                for (line, y) in text
                    .lines
                    .iter()
                    .skip(self.preview.index)
                    .zip(area.y..area.y + area.height)
                {
                    buf.set_line(area.x, y, line, area.width);
                }
            }
            Err(e) => {
                let span = Span::raw(format!("Failed to parse ansi colors: {}", e));
                buf.set_span(area.x, area.y, &span, area.width);
                let vec: Vec<&str> = s.split('\n').collect();

                for (line, y) in vec
                    .iter()
                    .skip(self.preview.index)
                    .zip(area.y + 1..area.y + area.height)
                {
                    let span = Span::raw(line.to_string());
                    buf.set_span(area.x, y, &span, area.width);
                }
            }
        }
    }
}

impl<'a> Widget for TuiFilePreview<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_text_preview(area, buf, self.preview.output.as_str());
    }
}
