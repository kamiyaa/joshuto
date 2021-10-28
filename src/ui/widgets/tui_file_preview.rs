use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
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
        let style = Style::default();
        let area_width = area.width as usize;
        for (y, s) in (area.y..area.y + area.height).zip(self.preview.output.as_str().split('\n')) {
            buf.set_stringn(area.x, y, s, area_width, style);
        }
    }
}
