use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::fs::{FileType, JoshutoDirEntry, JoshutoDirList};
use crate::ui::widgets::print_file_name;
use crate::util::style;

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

        if self.dirlist.contents.is_empty() {
            let style = Style::default().bg(Color::Red).fg(Color::White);
            buf.set_stringn(x, y, "empty", area.width as usize, style);
            return;
        }

        let curr_index = self.dirlist.index.unwrap();
        let skip_dist = curr_index / area.height as usize * area.height as usize;

        let drawing_width = area.width as usize;

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
        _ => print_file_name(buf, (x, y), name, style, drawing_width),
    }
}
