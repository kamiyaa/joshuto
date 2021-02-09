use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget, Wrap};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::context::JoshutoContext;
use crate::io::FileOp;
use crate::ui::widgets::TuiTopBar;

#[derive(Clone, Debug)]
struct IndexInfo {
    pub index: usize,
    pub x: usize,
    pub y: usize,
    pub c: char,
}

#[derive(Clone, Debug)]
struct LineInfo {
    pub start: usize,
    pub end: usize,
    pub width: usize,
}

pub struct TuiMultilineText<'a> {
    _s: &'a str,
    _width: usize,
    _lines: Vec<LineInfo>,
    _style: Style,
    _cursor_style: Style,
    _index_info: Option<IndexInfo>,
}

impl<'a> TuiMultilineText<'a> {
    pub fn new(s: &'a str, area_width: usize, index: Option<usize>) -> Self {
        let filter = |(i, c): (usize, char)| {
            let w = c.width()?;
            Some((i, c, w))
        };

        let mut lines = Vec::with_capacity(s.len() / area_width + 1);
        let mut start = 0;
        let mut end = 0;
        let mut line_width = 0;

        for (i, c, w) in s.char_indices().filter_map(filter) {
            end = i + c.len_utf8();
            if (line_width + w < area_width) {
                line_width += w;
                continue;
            }
            lines.push(LineInfo {
                start,
                end,
                width: line_width,
            });
            start = end;
            line_width = 0;
        }
        if (start < end) {
            lines.push(LineInfo {
                start,
                end: end,
                width: line_width,
            });
        }

        let mut index_info = None;
        if let Some(idx) = index {
            let (row, line_info) = lines
                .iter()
                .enumerate()
                .find(|(r, li)| li.start <= idx && li.end > idx)
                .unwrap();

            let mut s_width = 0;
            let substr = &s[line_info.start..line_info.end];
            for (i, c, w) in substr.char_indices().filter_map(filter) {
                if (line_info.start + i <= idx) {
                    s_width += w;
                    continue;
                }
                index_info = Some(IndexInfo {
                    index: idx,
                    x: s_width,
                    y: row,
                    c: c,
                });
                break;
            }

            if let None = index_info {
                let s_width = substr.width();
                index_info = Some(IndexInfo {
                    index: idx,
                    x: s_width % area_width,
                    y: row + 1,
                    c: ' ',
                });
            }
        }

        let default_style = Style::default().fg(Color::Reset).bg(Color::Reset);
        let cursor_style = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::REVERSED);
        Self {
            _s: s,
            _lines: lines,
            _width: area_width,
            _style: default_style,
            _cursor_style: cursor_style,
            _index_info: index_info,
        }
    }

    pub fn width(&self) -> usize {
        self._width
    }

    pub fn len(&self) -> usize {
        match self._index_info.as_ref() {
            Some(index_info) if index_info.y >= self._lines.len() => index_info.y + 1,
            _ => self._lines.len(),
        }
    }

    pub fn style(&mut self, style: Style) {
        self._style = style;
    }

    pub fn cursor_style(&mut self, style: Style) {
        self._cursor_style = style;
    }
}

impl<'a> Widget for TuiMultilineText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area_left = area.left();
        let area_top = area.top();
        for (i, line_info) in self._lines.iter().enumerate().take(self._lines.len() - 1) {
            buf.set_string(
                area_left,
                area_top + i as u16,
                &self._s[line_info.start..line_info.end],
                self._style,
            );
        }
        let line_info = &self._lines[self._lines.len() - 1];
        buf.set_string(
            area_left,
            area_top + (self._lines.len() - 1) as u16,
            &self._s[line_info.start..line_info.end],
            self._style,
        );

        if let Some(index_info) = self._index_info {
            buf.set_string(
                area_left + index_info.x as u16,
                area_top + index_info.y as u16,
                index_info.c.to_string(),
                self._cursor_style,
            );
        }
    }
}
