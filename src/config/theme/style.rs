use ratatui::style::{self, Style};

const fn default_color() -> style::Color {
    style::Color::Reset
}

/// A resolved fg/bg color, text prefix, and modifier set for one themed UI element.
#[derive(Clone, Debug)]
pub struct AppStyle {
    pub fg: style::Color,
    pub bg: style::Color,
    pub prefix: String,
    pub modifier: style::Modifier,
}

impl AppStyle {
    /// Returns `self` with the background color set, for builder-style construction.
    pub fn set_bg(mut self, bg: style::Color) -> Self {
        self.bg = bg;
        self
    }
    /// Returns `self` with the foreground color set, for builder-style construction.
    pub fn set_fg(mut self, fg: style::Color) -> Self {
        self.fg = fg;
        self
    }
    /// Returns `self` with the text prefix set, for builder-style construction.
    pub fn set_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }

    /// Returns `self` with `modifier` added to the existing modifiers.
    pub fn insert(mut self, modifier: style::Modifier) -> Self {
        self.modifier.insert(modifier);
        self
    }

    /// Converts to a ratatui [`Style`] for rendering.
    pub fn as_style(&self) -> Style {
        Style::from(self)
    }
}

impl std::default::Default for AppStyle {
    fn default() -> Self {
        Self {
            fg: default_color(),
            bg: default_color(),
            prefix: String::new(),
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
