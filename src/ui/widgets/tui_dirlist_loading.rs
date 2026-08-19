use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;

/// Placeholder widget shown in place of a directory pane while its listing is still loading.
pub struct TuiDirListLoading;

impl TuiDirListLoading {
    /// Creates the loading-placeholder widget.
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for TuiDirListLoading {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 1 {
            return;
        }
        let x = area.left();
        let y = area.top();

        let style = Style::default().fg(Color::Yellow);
        buf.set_stringn(x, y, "loading...", area.width as usize, style);
    }
}
