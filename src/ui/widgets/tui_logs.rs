use lazy_static::lazy_static;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Widget;

lazy_static! {
    static ref TITLE_STYLE: Style = Style::default().add_modifier(Modifier::REVERSED);
    static ref DEFAULT_STYLE: Style = Style::default();
    static ref ERROR_STYLE: Style = Style::default().fg(Color::Red);
    static ref WARN_STYLE: Style = Style::default().fg(Color::Yellow);
    static ref DEBUG_STYLE: Style = Style::default().fg(Color::DarkGray);
}

const TITLE: &str = "Logs";
const FOOTER: &str = "Press <ESC> to return, arrows/page up/page down to scroll";

/// The scrollable view of everything `tracing` has logged this session.
pub struct TuiLogs<'a> {
    lines: &'a [&'a str],
    offset: &'a mut usize,
}

impl<'a> TuiLogs<'a> {
    /// Creates the log widget with `offset` as the index of the top visible line. `offset` is
    /// clamped while rendering so the last page is never scrolled past.
    pub fn new(lines: &'a [&'a str], offset: &'a mut usize) -> Self {
        Self { lines, offset }
    }

    /// Number of log lines visible in a view of total height `height`, after the title and
    /// footer rows.
    pub fn page_height(height: u16) -> usize {
        height.saturating_sub(2) as usize
    }
}

impl Widget for TuiLogs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 2 {
            return;
        }
        let width = area.width as usize;
        let page_height = Self::page_height(area.height);

        let max_offset = self.lines.len().saturating_sub(page_height);
        if *self.offset > max_offset {
            *self.offset = max_offset;
        }

        buf.set_stringn(
            area.x,
            area.y,
            format!("{:^w$}", TITLE, w = width),
            width,
            *TITLE_STYLE,
        );

        let visible = self.lines.iter().skip(*self.offset).take(page_height);
        for (i, line) in visible.enumerate() {
            buf.set_stringn(area.x, area.y + 1 + i as u16, line, width, line_style(line));
        }

        let footer = if self.lines.is_empty() {
            "No logs yet. Press <ESC> to return".to_string()
        } else {
            let last = (*self.offset + page_height).min(self.lines.len());
            format!(
                "{FOOTER} ({}-{} of {})",
                *self.offset + 1,
                last,
                self.lines.len()
            )
        };
        buf.set_stringn(
            area.x,
            area.bottom() - 1,
            format!("{:^w$}", footer, w = width),
            width,
            *TITLE_STYLE,
        );
    }
}

/// Picks a color from the level `tracing_subscriber::fmt` writes near the start of each line.
fn line_style(line: &str) -> Style {
    let head: String = line.chars().take(40).collect();
    if head.contains("ERROR") {
        *ERROR_STYLE
    } else if head.contains("WARN") {
        *WARN_STYLE
    } else if head.contains("DEBUG") || head.contains("TRACE") {
        *DEBUG_STYLE
    } else {
        *DEFAULT_STYLE
    }
}

#[cfg(test)]
mod tests {
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::widgets::Widget;

    use super::TuiLogs;

    fn row(buf: &Buffer, y: u16) -> String {
        (0..buf.area.width)
            .map(|x| buf[(x, y)].symbol())
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    #[test]
    fn offset_past_the_end_is_clamped_to_the_last_page() {
        let lines = ["one", "two", "three", "four", "five"];
        let area = Rect::new(0, 0, 80, 4);
        let mut buf = Buffer::empty(area);
        let mut offset = usize::MAX;

        TuiLogs::new(&lines, &mut offset).render(area, &mut buf);

        assert_eq!(offset, 3);
        assert_eq!(row(&buf, 1), "four");
        assert_eq!(row(&buf, 2), "five");
    }

    #[test]
    fn renders_from_the_top_when_offset_is_zero() {
        let lines = ["one", "two", "three"];
        let area = Rect::new(0, 0, 80, 4);
        let mut buf = Buffer::empty(area);
        let mut offset = 0;

        TuiLogs::new(&lines, &mut offset).render(area, &mut buf);

        assert_eq!(offset, 0);
        assert_eq!(row(&buf, 1), "one");
        assert_eq!(row(&buf, 2), "two");
    }
}
