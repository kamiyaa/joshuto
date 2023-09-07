use ratatui::style::{self, Style};

const fn default_color() -> style::Color {
    style::Color::Reset
}

#[derive(Clone, Debug)]
pub struct AppStyle {
    pub fg: style::Color,
    pub bg: style::Color,
    pub modifier: style::Modifier,
}

impl AppStyle {
    pub fn set_bg(mut self, bg: style::Color) -> Self {
        self.bg = bg;
        self
    }
    pub fn set_fg(mut self, fg: style::Color) -> Self {
        self.fg = fg;
        self
    }

    pub fn insert(mut self, modifier: style::Modifier) -> Self {
        self.modifier.insert(modifier);
        self
    }

    pub fn as_style(&self) -> Style {
        Style::from(self)
    }
}

impl std::default::Default for AppStyle {
    fn default() -> Self {
        Self {
            fg: default_color(),
            bg: default_color(),
            modifier: style::Modifier::empty(),
        }
    }
}

impl From<&AppStyle> for Style {
    fn from(style: &AppStyle) -> Self {
        Self::default()
            .fg(style.fg)
            .bg(style.bg)
            .add_modifier(style.modifier)
    }
}
