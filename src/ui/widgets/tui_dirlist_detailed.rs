use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::fs::{FileType, JoshutoDirEntry, JoshutoDirList};
use crate::util::format;
use crate::util::style;

const FILE_SIZE_WIDTH: usize = 8;
const SYMLINK_WIDTH: usize = 4;

const ELLIPSIS: &str = "…";

pub struct TuiDirListDetailed<'a> {
    dirlist: &'a JoshutoDirList,
}

impl<'a> TuiDirListDetailed<'a> {
    pub fn new(dirlist: &'a JoshutoDirList) -> Self {
        Self { dirlist }
    }
}

impl<'a> Widget for TuiDirListDetailed<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 1 {
            return;
        }

        let x = area.left();
        let y = area.top();
        let curr_index = match self.dirlist.index {
            Some(i) => i,
            None => {
                let style = Style::default().bg(Color::Red).fg(Color::White);
                buf.set_stringn(x, y, "empty", area.width as usize, style);
                return;
            }
        };

        let drawing_width = area.width as usize;
        let skip_dist = curr_index / area.height as usize * area.height as usize;

        // draw every entry
        self.dirlist
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
            .for_each(|(i, entry)| {
                let style = style::entry_style(entry);
                print_entry(buf, entry, style, (x + 1, y + i as u16), drawing_width - 1);
            });

        // draw selected entry in a different style
        let screen_index = curr_index % area.height as usize;

        let entry = self.dirlist.curr_entry_ref().unwrap();
        let style = style::entry_style(entry).add_modifier(Modifier::REVERSED);

        let space_fill = " ".repeat(drawing_width);
        buf.set_string(x, y + screen_index as u16, space_fill.as_str(), style);

        print_entry(
            buf,
            entry,
            style,
            (x + 1, y + screen_index as u16),
            drawing_width - 1,
        );
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
            buf.set_stringn(x, y, name, drawing_width, style);
            if name_width > drawing_width {
                buf.set_string(x + drawing_width as u16 - 1, y, ELLIPSIS, style);
            }
        }
        FileType::Symlink(_) => {
            if drawing_width < SYMLINK_WIDTH {
                return;
            }
            let file_drawing_width = drawing_width - SYMLINK_WIDTH;

            print_file_name(buf, (x, y), name, style, file_drawing_width);

            // print arrow
            buf.set_string(x + file_drawing_width as u16, y, "->", style);
            if name_width >= file_drawing_width {
                buf.set_string(x + file_drawing_width as u16 - 1, y, ELLIPSIS, style);
            }
        }
        FileType::File => {
            if drawing_width < FILE_SIZE_WIDTH {
                return;
            }
            let file_drawing_width = drawing_width - FILE_SIZE_WIDTH;

            print_file_name(buf, (x, y), name, style, file_drawing_width);

            // print file size
            let file_size_string = format::file_size_to_string(entry.metadata.len());
            buf.set_string(x + file_drawing_width as u16, y, " ", style);
            buf.set_string(
                x + file_drawing_width as u16 + 1,
                y,
                file_size_string,
                style,
            );
        }
    }
}

pub fn print_file_name(
    buf: &mut Buffer,
    (x, y): (u16, u16),
    name: &str,
    style: Style,
    drawing_width: usize,
) {
    let (stem, extension) = match name.rfind('.') {
        None => (name, ""),
        Some(i) => name.split_at(i),
    };
    if stem.is_empty() {
        let ext_width = extension.width();
        buf.set_stringn(x, y, extension, drawing_width, style);
        if ext_width > drawing_width {
            buf.set_string(x + drawing_width as u16 - 1, y, ELLIPSIS, style);
        }
    } else if extension.is_empty() {
        let stem_width = stem.width();
        buf.set_stringn(x, y, stem, drawing_width, style);
        if stem_width > drawing_width {
            buf.set_string(x + drawing_width as u16 - 1, y, ELLIPSIS, style);
        }
    } else {
        if stem.is_empty() {
            let ext_width = extension.width();
            buf.set_stringn(x, y, extension, drawing_width, style);
            if ext_width > drawing_width {
                buf.set_string(x + drawing_width as u16 - 1, y, ELLIPSIS, style);
            }
        } else if extension.is_empty() {
            let stem_width = stem.width();
            buf.set_stringn(x, y, stem, drawing_width, style);
            if stem_width > drawing_width {
                buf.set_string(x + drawing_width as u16 - 1, y, ELLIPSIS, style);
            }
        } else {
            let stem_width = stem.width();
            let ext_width = extension.width();
            if stem_width + ext_width <= drawing_width {
                // file stem and extension fits
                buf.set_stringn(x, y, stem, drawing_width, style);
                buf.set_string(x + stem_width as u16, y, extension, style);
            } else if ext_width > drawing_width {
                // file ext does not fit
                buf.set_string(x, y, ELLIPSIS, style);
            } else {
                buf.set_stringn(x, y, stem, drawing_width, style);
                // file ext fits, but file stem does not
                let ext_start_idx = if drawing_width < ext_width {
                    0
                } else {
                    (drawing_width - ext_width) as u16
                };
                buf.set_string(x + ext_start_idx, y, extension, style);
                let ext_start_idx = if ext_start_idx > 0 {
                    ext_start_idx - 1
                } else {
                    0
                };
                buf.set_string(x + ext_start_idx, y, ELLIPSIS, style);
            }
        }
    }
}
