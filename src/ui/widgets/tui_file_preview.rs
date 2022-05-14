use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::Widget;

use crate::preview::preview_file::FilePreview;

fn color_to_color(color: tui_old::style::Color) -> Color {
    match color {
        tui_old::style::Color::Reset => Color::Reset,
        tui_old::style::Color::Black => Color::Black,
        tui_old::style::Color::Red => Color::Red,
        tui_old::style::Color::Green => Color::Green,
        tui_old::style::Color::Yellow => Color::Yellow,
        tui_old::style::Color::Blue => Color::Blue,
        tui_old::style::Color::Magenta => Color::Magenta,
        tui_old::style::Color::Cyan => Color::Cyan,
        tui_old::style::Color::Gray => Color::Gray,
        tui_old::style::Color::DarkGray => Color::DarkGray,
        tui_old::style::Color::LightRed => Color::LightRed,
        tui_old::style::Color::LightGreen => Color::LightGreen,
        tui_old::style::Color::LightYellow => Color::LightYellow,
        tui_old::style::Color::LightBlue => Color::LightBlue,
        tui_old::style::Color::LightMagenta => Color::LightMagenta,
        tui_old::style::Color::LightCyan => Color::LightCyan,
        tui_old::style::Color::White => Color::White,
        tui_old::style::Color::Rgb(r, g, b) => Color::Rgb(r, g, b),
        tui_old::style::Color::Indexed(i) => Color::Indexed(i),
    }
}

fn style_to_style(style: tui_old::style::Style) -> Style {
    let mut new_style = Style::default();
    if let Some(fg) = style.fg {
        new_style = new_style.fg(color_to_color(fg));
    }
    if let Some(bg) = style.bg {
        new_style = new_style.fg(color_to_color(bg));
    }
    new_style
}

pub struct TuiFilePreview<'a> {
    preview: &'a FilePreview,
}

impl<'a> TuiFilePreview<'a> {
    pub fn new(preview: &'a FilePreview) -> Self {
        Self { preview }
    }

    #[cfg(not(feature = "syntax_highlight"))]
    fn render_text_preview(&self, area: Rect, buf: &mut Buffer, s: &str) {
        let vec: Vec<&str> = s.split('\n').collect();
        for (line, y) in vec
            .iter()
            .skip(self.preview.index)
            .zip(area.y..area.y + area.height)
        {
            let span = Span::raw(line.to_string());
            buf.set_span(area.x, y, &span, area.width);
        }
    }

    #[cfg(feature = "syntax_highlight")]
    fn render_text_preview(&self, area: Rect, buf: &mut Buffer, s: &str) {
        use ansi_to_tui::ansi_to_text;
        let vec = s.as_bytes().to_vec();
        let res = ansi_to_text(vec);

        match res {
            Ok(text) => {
                for (line, y) in text
                    .lines
                    .iter()
                    .skip(self.preview.index)
                    .zip(area.y..area.y + area.height)
                {
                    // required to convert between different tui-rs versions.
                    // remove once ansi-to-tui depends on latest tui-rs
                    let span_vec: Vec<Span> = line
                        .0
                        .iter()
                        .map(|s| Span {
                            content: s.content.clone(),
                            style: style_to_style(s.style),
                        })
                        .collect();
                    let spans = Spans(span_vec);
                    let line = &spans;

                    buf.set_spans(area.x, y, line, area.width);
                }
            }
            Err(e) => {
                let span = Span::raw(format!("Failed to parse ansi colors: {}", e));
                buf.set_span(area.x, area.y, &span, area.width);
                let vec: Vec<&str> = s.split('\n').collect();

                for (line, y) in vec
                    .iter()
                    .skip(self.preview.index)
                    .zip(area.y + 1..area.y + area.height)
                {
                    let span = Span::raw(line.to_string());
                    buf.set_span(area.x, y, &span, area.width);
                }
            }
        }
    }
}

impl<'a> Widget for TuiFilePreview<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_text_preview(area, buf, self.preview.output.as_str());
    }
}
