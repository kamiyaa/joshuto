use ansi_to_tui::ansi_to_text;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::text::Text;
use tui::widgets::Widget;

use crate::fs::JoshutoDirEntry;
use crate::preview::preview_file::FilePreview;

pub struct TuiFilePreview<'a> {
    _entry: &'a JoshutoDirEntry,
    preview: &'a FilePreview,
}

impl<'a> TuiFilePreview<'a> {
    pub fn new(_entry: &'a JoshutoDirEntry, preview: &'a FilePreview) -> Self {
        Self { _entry, preview }
    }
}

impl<'a> Widget for TuiFilePreview<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text: Text = ansi_to_text(self.preview.output.as_str().as_bytes().to_vec()).unwrap();
        for (y, line) in (area.y..area.y + area.height).zip(text.lines) {
            buf.set_spans(area.x, y, &line, area.width);
        }
    }
}
