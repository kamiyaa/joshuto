use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;

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
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
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
        for (i, entry) in self
            .dirlist
            .contents
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
        {
            let name = entry.file_name();
            let mut style = entry.get_style();

            if i == screen_index {
                style = style.modifier(Modifier::REVERSED);
            }
            let name_width = name.width();
            if name_width < area_width {
                buf.set_stringn(x, y + i as u16,
                        name,
                        area_width, style);
            } else {
                match name.rfind('.') {
                    None => {
                        buf.set_stringn(x, y + i as u16,
                            name,
                            area_width, style);
                    }
                    Some(p_ind) => {
                        let ext_width = name[p_ind..].width();
                        let file_name_width = area_width - ext_width - 1;

                        buf.set_stringn(x, y + i as u16,
                            &name[..p_ind],
                            file_name_width, style);
                        buf.set_string(x + file_name_width as u16, y + i as u16,
                            "…", style);
                        buf.set_string(x + file_name_width as u16 + 1, y + i as u16,
                            &name[p_ind..], style);
                    }
                }
            }
        }
    }
}

const FILE_SIZE_WIDTH: usize = 8;

pub struct TuiDirListDetailed<'a> {
    dirlist: &'a JoshutoDirList,
}

impl<'a> TuiDirListDetailed<'a> {
    pub fn new(dirlist: &'a JoshutoDirList) -> Self {
        Self { dirlist }
    }
}

impl<'a> Widget for TuiDirListDetailed<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
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
            let style = Style::default()
                .bg(Color::Red)
                .fg(Color::White);
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

        let area_width = area.width as usize;
        for (i, entry) in self
            .dirlist
            .contents
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
        {
            let name = entry.file_name();
            let mut style = entry.get_style();

            if i == screen_index {
                style = style.modifier(Modifier::REVERSED);
            }
            let file_type = entry.metadata.file_type;

            let name_width = name.width();
            if file_type.is_dir() {
                buf.set_stringn(x, y + i as u16,
                        name,
                        area_width, style);
                continue;
            }

            if name_width < area_width - FILE_SIZE_WIDTH {
                buf.set_stringn(x, y + i as u16,
                        name,
                        area_width - FILE_SIZE_WIDTH, style);
            } else {
                match name.rfind('.') {
                    None => {
                        buf.set_stringn(x, y + i as u16,
                            name,
                            area_width - FILE_SIZE_WIDTH, style);
                    }
                    Some(p_ind) => {
                        let ext_width = name[p_ind..].width();
                        let file_name_width = area_width - FILE_SIZE_WIDTH - ext_width - 1;

                        buf.set_stringn(x, y + i as u16,
                            &name[..p_ind],
                            file_name_width, style);
                        buf.set_string(x + file_name_width as u16, y + i as u16,
                            "…", style);
                        buf.set_string(x + file_name_width as u16 + 1, y + i as u16,
                            &name[p_ind..], style);
                    }
                }
            }
            let file_size_string = file_size_to_string(entry.metadata.len as f64);
            buf.set_string(x + (area_width - FILE_SIZE_WIDTH) as u16, y + i as u16,
                file_size_string, style);
        }
    }
}

fn file_size_to_string(mut file_size: f64) -> String {
    const FILE_UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "E"];
    const CONV_RATE: f64 = 1024.0;

    let mut index = 0;
    while file_size > CONV_RATE {
        file_size /= CONV_RATE;
        index += 1;
    }

    if file_size >= 100.0 {
        format!("{:>4.0} {}", file_size, FILE_UNITS[index])
    } else if file_size >= 10.0 {
        format!("{:>4.1} {}", file_size, FILE_UNITS[index])
    } else {
        format!("{:>4.2} {}", file_size, FILE_UNITS[index])
    }
}
