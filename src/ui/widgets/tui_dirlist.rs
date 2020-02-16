use std::io::{self, Write};

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;

use crate::fs::JoshutoDirList;
/*
pub struct TermionWindow<'a> {
    rect: Rect,
    dirlist: &'a JoshutoDirList,
}

impl<'a> TermionWindow<'a> {
    pub fn new(rect: &Rect, dirlist: &'a JoshutoDirList) -> Self {
        Self {
            rect: rect.clone(),
            dirlist
        }
    }

    pub fn update_rect(&mut self, rect: &Rect) {
        self.rect = rect.clone();
    }

    pub fn clear<W: Write>(&self, terminal: &mut W) {
        for i in 1..self.rect.height + 1 {
            write!(terminal, "{}{}", Goto(self.rect.width, i as u16), clear::BeforeCursor);
        }
    }

    pub fn draw<W: Write>(&self, terminal: &mut W) {
        if self.rect.height < 4 {
            return;
        }
        let dir_len = self.dirlist.contents.len();
        if dir_len == 0 {
            write!(terminal, "{}EMPTY", Goto(self.rect.x, self.rect.y));
            return;
        }

        let curr_index = self.dirlist.index.unwrap();

        let height = self.rect.height as usize;

        for (i, entry) in self.dirlist.contents.iter().enumerate() {
            let goto_i = i + 1;
            if goto_i > height {
                break;
            }
            let fg = entry.get_fg_color();
            let bg = entry.get_bg_color();
            let file_ansi_text = entry.as_ansi_text();

            if i == curr_index {
                write!(terminal, "{}{}{}{}{}",
                    Goto(self.rect.x, goto_i as u16),
                    style::Invert,
                    bg.bg_str(),
                    fg.fg_str(),
                    file_ansi_text);
            } else {
                write!(terminal, "{}{}{}{}{}",
                    Goto(self.rect.x, goto_i as u16),
                    style::Reset,
                    bg.bg_str(),
                    fg.fg_str(),
                    file_ansi_text);
            }
        }
    }
}
*/
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
            buf.set_stringn(x, y + i as u16, name, area.width as usize, style);
        }
    }
}
