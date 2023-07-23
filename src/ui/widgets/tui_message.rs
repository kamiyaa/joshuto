use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

pub struct TuiMessage<'a> {
    message: &'a str,
    style: Style,
}

impl<'a> TuiMessage<'a> {
    pub fn new(message: &'a str, style: Style) -> Self {
        Self { message, style }
    }
}

impl<'a> Widget for TuiMessage<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 1 {
            return;
        }
        let x = area.left();
        let y = area.top();

        buf.set_stringn(x, y, self.message, area.width as usize, self.style);
    }
}
