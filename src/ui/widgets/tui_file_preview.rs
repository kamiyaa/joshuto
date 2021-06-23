use std::process;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;

use crate::fs::JoshutoDirEntry;
use crate::preview::preview_file::FilePreview;
use crate::util::format;
use crate::util::string::UnicodeTruncate;
use crate::util::style;
use unicode_width::UnicodeWidthStr;

const MIN_LEFT_LABEL_WIDTH: i32 = 15;

const ELLIPSIS: &str = "…";

pub struct TuiFilePreview<'a> {
    entry: &'a JoshutoDirEntry,
    preview: &'a FilePreview,
}

impl<'a> TuiFilePreview<'a> {
    pub fn new(entry: &'a JoshutoDirEntry, preview: &'a FilePreview) -> Self {
        Self { entry, preview }
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
