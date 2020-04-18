use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::fs::JoshutoDirList;

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

        let dir_len = self.dirlist.contents.len();
        if dir_len == 0 {
            let style = Style::default().bg(Color::Red).fg(Color::White);
            buf.set_stringn(x, y, "empty", area.width as usize, style);
            return;
        }

        let curr_index = self.dirlist.index.unwrap();
        let skip_dist = curr_index / area.height as usize * area.height as usize;

        let screen_index = if skip_dist > 0 {
            curr_index % skip_dist
        } else {
            curr_index
        };

        let area_width = area.width as usize - 1;
        for (i, entry) in self.dirlist.contents[skip_dist..]
            .iter()
            .enumerate()
            .take(area.height as usize)
        {
            let name = entry.file_name();
            let name_width = name.width();

            let style = if i == screen_index {
                entry.get_style().modifier(Modifier::REVERSED)
            } else {
                entry.get_style()
            };

            let file_type = &entry.metadata.file_type;
            if file_type.is_dir() {
                if name_width <= area_width {
                    buf.set_stringn(x, y + i as u16, name, area_width, style);
                } else {
                    buf.set_stringn(x, y + i as u16, name, area_width - 1, style);
                    buf.set_string(x + area_width as u16 - 1, y + i as u16, "…", style);
                }
            } else {
                if name_width < area_width {
                    buf.set_stringn(x, y + i as u16, name, area_width, style);
                } else {
                    match name.rfind('.') {
                        None => {
                            buf.set_stringn(x, y + i as u16, name, area_width, style);
                        }
                        Some(p_ind) => {
                            let ext_width = name[p_ind..].width();
                            let file_name_width = area_width - ext_width - 1;

                            buf.set_stringn(
                                x,
                                y + i as u16,
                                &name[..p_ind],
                                file_name_width,
                                style,
                            );
                            buf.set_string(x + file_name_width as u16, y + i as u16, "…", style);
                            buf.set_string(
                                x + file_name_width as u16 + 1,
                                y + i as u16,
                                &name[p_ind..],
                                style,
                            );
                        }
                    }
                }
            }
        }
    }
}
