use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Clone, Debug)]
pub struct LineInfo {
    pub start: usize,
    pub end: usize,
    pub width: usize,
}

pub struct TuiMultilineText<'a> {
    _s: &'a str,
    _width: usize,
    _lines: Vec<LineInfo>,
    _style: Style,
}

impl<'a> TuiMultilineText<'a> {
    pub fn new(s: &'a str, area_width: usize) -> Self {
        // TODO: This is a very hacky way of doing it and I would like
        // to clean this up more

        let default_style = Style::default().fg(Color::Reset).bg(Color::Reset);

        let s_width = s.width();
        if s_width < area_width {
            return Self {
                _s: s,
                _lines: vec![LineInfo {
                    start: 0,
                    end: s.len(),
                    width: s_width,
                }],
                _width: area_width,
                _style: default_style,
            };
        }

        let filter = |(i, c): (usize, char)| {
            let w = c.width()?;
            Some((i, c, w))
        };

        let mut lines = Vec::with_capacity(s.len() / area_width);

        let mut start = 0;
        let mut line_width = 0;
        for (i, _, w) in s.char_indices().filter_map(filter) {
            if line_width + w < area_width {
                line_width += w;
                continue;
            }
            lines.push(LineInfo {
                start,
                end: i,
                width: line_width,
            });
            line_width = w;
            start = i;
        }
        lines.push(LineInfo {
            start,
            end: s.len(),
            width: s[start..s.len()].width(),
        });

        Self {
            _s: s,
            _lines: lines,
            _width: area_width,
            _style: default_style,
        }
    }

    pub fn width(&self) -> usize {
        self._width
    }

    pub fn height(&self) -> usize {
        if self._lines[self._lines.len() - 1].width >= self.width() {
            return self.len() + 1;
        }
        self.len()
    }
    pub fn len(&self) -> usize {
        self._lines.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &LineInfo> {
        self._lines.iter()
    }
}

impl<'a> Widget for TuiMultilineText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area_left = area.left();
        let area_top = area.top();
        for (i, line_info) in self.iter().enumerate() {
            buf.set_string(
                area_left,
                area_top + i as u16,
                &self._s[line_info.start..line_info.end],
                self._style,
            );
        }
    }
}
