use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::fs::{FileType, JoshutoDirEntry, JoshutoDirList};

const ELLIPSIS: &str = "…";

pub struct TuiDirList<'a> {
    dirlist: &'a JoshutoDirList,
}

impl<'a> TuiDirList<'a> {
    pub fn new(dirlist: &'a JoshutoDirList) -> Self {
        Self { dirlist }
    }
}

impl<'a> Widget for TuiDirList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 1 || area.height < 1 {
            return;
        }
        if area.width < 4 {
            return;
        }
        let x = area.left();
        let y = area.top();

        if self.dirlist.contents.len() == 0 {
            let style = Style::default().bg(Color::Red).fg(Color::White);
            buf.set_stringn(x, y, "empty", area.width as usize, style);
            return;
        }

        let curr_index = self.dirlist.index.unwrap();
        let skip_dist = curr_index / area.height as usize * area.height as usize;

        let drawing_width = area.width as usize;
        for (i, entry) in self
            .dirlist
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
        {
            let style = entry.get_style();
            print_entry(buf, entry, style, (x + 1, y + i as u16), drawing_width - 1);
        }
        {
            let screen_index = curr_index % area.height as usize;

            let entry = self.dirlist.curr_entry_ref().unwrap();
            let style = {
                let s = entry.get_style().add_modifier(Modifier::REVERSED);
                let space_fill = " ".repeat(drawing_width);
                buf.set_string(x, y + screen_index as u16, space_fill.as_str(), s);
                s
            };
            print_entry(
                buf,
                entry,
                style,
                (x + 1, y + screen_index as u16),
                drawing_width - 1,
            );
        }
    }
}

fn print_entry(
    buf: &mut Buffer,
    entry: &JoshutoDirEntry,
    style: Style,
    (x, y): (u16, u16),
    drawing_width: usize,
) {
    let name = entry.label();
    let name_width = name.width();

    match entry.metadata.file_type() {
        FileType::Directory => {
            // print filename
            buf.set_stringn(x, y, name, drawing_width, style);
            if name_width > drawing_width {
                buf.set_string(x + drawing_width as u16 - 1, y, ELLIPSIS, style);
            }
        }
        _ => {
            let file_drawing_width = drawing_width;
            let (stem, extension) = match name.rfind('.') {
                None => (name, ""),
                Some(i) => name.split_at(i),
            };
            if stem.is_empty() {
                let ext_width = extension.width();
                buf.set_stringn(x, y, extension, file_drawing_width, style);
                if ext_width > drawing_width {
                    buf.set_string(x + drawing_width as u16 - 1, y, ELLIPSIS, style);
                }
            } else if extension.is_empty() {
                let stem_width = stem.width();
                buf.set_stringn(x, y, stem, file_drawing_width, style);
                if stem_width > file_drawing_width {
                    buf.set_string(x + file_drawing_width as u16 - 1, y, ELLIPSIS, style);
                }
            } else {
                let stem_width = stem.width();
                let ext_width = extension.width();
                buf.set_stringn(x, y, stem, file_drawing_width, style);
                if stem_width + ext_width > file_drawing_width {
                    let ext_start_idx = if file_drawing_width < ext_width {
                        0
                    } else {
                        (file_drawing_width - ext_width) as u16
                    };
                    buf.set_string(x + ext_start_idx, y, extension, style);
                    let ext_start_idx = if ext_start_idx > 0 {
                        ext_start_idx - 1
                    } else {
                        0
                    };
                    buf.set_string(x + ext_start_idx, y, ELLIPSIS, style);
                } else {
                    buf.set_string(x + stem_width as u16, y, extension, style);
                }
            }
        }
    }
}
