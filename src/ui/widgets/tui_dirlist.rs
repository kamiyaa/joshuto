use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::fs::{JoshutoDirEntry, JoshutoDirList};
use crate::ui::widgets::trim_file_label;
use crate::util::style;

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
        if area.width < 4 || area.height < 1 {
            return;
        }
        let x = area.left();
        let y = area.top();

        if self.dirlist.contents.is_empty() {
            let style = Style::default().bg(Color::Red).fg(Color::White);
            buf.set_stringn(x, y, "empty", area.width as usize, style);
            return;
        }

        let curr_index = self.dirlist.get_index().unwrap();
        let skip_dist = self.dirlist.first_index_for_viewport();

        let drawing_width = area.width as usize;

        self.dirlist
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
            .for_each(|(i, entry)| {
                let ix = skip_dist + i;

                let style = if ix == curr_index {
                    style::entry_style(entry).add_modifier(Modifier::REVERSED)
                } else {
                    style::entry_style(entry)
                };

                if ix == curr_index {
                    let space_fill = " ".repeat(drawing_width);
                    buf.set_string(x, y + i as u16, space_fill.as_str(), style);
                }

                print_entry(buf, entry, style, (x + 1, y + i as u16), drawing_width - 1);
            });
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
    let label = if name_width > drawing_width {
        trim_file_label(name, drawing_width)
    } else {
        name.to_string()
    };
    buf.set_string(x, y, label, style);
}
