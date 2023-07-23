use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::fs::{JoshutoDirEntry, JoshutoDirList};
use crate::ui::widgets::trim_file_label;
use crate::util::style;

pub struct TuiDirList<'a> {
    dirlist: &'a JoshutoDirList,
    pub focused: bool,
}

impl<'a> TuiDirList<'a> {
    pub fn new(dirlist: &'a JoshutoDirList, focused: bool) -> Self {
        Self { dirlist, focused }
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

        let space_fill = " ".repeat(drawing_width);

        self.dirlist
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
            .for_each(|(i, entry)| {
                let ix = skip_dist + i;

                let style = if !self.focused {
                    style::entry_style(entry)
                } else if ix == curr_index {
                    style::entry_style(entry).add_modifier(Modifier::REVERSED)
                } else {
                    style::entry_style(entry)
                };

                buf.set_string(x, y + i as u16, space_fill.as_str(), style);

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
