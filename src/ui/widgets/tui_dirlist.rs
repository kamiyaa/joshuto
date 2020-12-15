use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::fs::{FileType, JoshutoDirList};

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
        let screen_index = curr_index % area.height as usize;

        let drawing_width = area.width as usize - 2;
        let space_fill = " ".repeat(drawing_width + 1);

        let x_start = x + 1;

        for (i, entry) in self
            .dirlist
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
        {
            let name = entry.label();
            let name_width = name.width();

            let style = if i == screen_index {
                let s = entry.get_style().add_modifier(Modifier::REVERSED);
                buf.set_string(x, y + i as u16, space_fill.as_str(), s);
                s
            } else {
                entry.get_style()
            };

            match entry.metadata.file_type() {
                FileType::Directory => {
                    if name_width <= drawing_width {
                        buf.set_stringn(x_start, y + i as u16, name, drawing_width, style);
                    } else {
                        buf.set_stringn(x_start, y + i as u16, name, drawing_width - 1, style);
                        buf.set_string(
                            x_start + drawing_width as u16 - 1,
                            y + i as u16,
                            "…",
                            style,
                        );
                    }
                }
                _ => {
                    if name_width < drawing_width {
                        buf.set_stringn(x_start, y + i as u16, name, drawing_width, style);
                    } else {
                        match name.rfind('.') {
                            None => {
                                buf.set_stringn(x_start, y + i as u16, name, drawing_width, style);
                            }
                            Some(p_ind) => {
                                let ext_width = name[p_ind..].width();
                                let file_name_width = drawing_width - ext_width - 1;

                                buf.set_stringn(
                                    x_start,
                                    y + i as u16,
                                    &name[..p_ind],
                                    file_name_width,
                                    style,
                                );
                                buf.set_string(
                                    x_start + file_name_width as u16,
                                    y + i as u16,
                                    "…",
                                    style,
                                );
                                buf.set_stringn(
                                    x_start + file_name_width as u16 + 1,
                                    y + i as u16,
                                    &name[p_ind..],
                                    drawing_width - file_name_width,
                                    style,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
