use ratatui::style;

use crate::{config::theme::app_style_raw::AppStyleRaw, utils::style_helper};

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
    pub fn as_style(&self) -> style::Style {
        style::Style::from(self)
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

impl From<&AppStyle> for style::Style {
    fn from(style: &AppStyle) -> Self {
        Self::default()
            .fg(style.fg)
            .bg(style.bg)
            .add_modifier(style.modifier)
    }
}

impl From<AppStyleRaw> for AppStyle {
    fn from(value: AppStyleRaw) -> Self {
        let bg = style_helper::str_to_color(value.bg.as_str());
        let fg = style_helper::str_to_color(value.fg.as_str());

        let mut modifier = style::Modifier::empty();
        if value.bold {
            modifier.insert(style::Modifier::BOLD);
        }
        if value.underline {
            modifier.insert(style::Modifier::UNDERLINED);
        }
        if value.invert {
            modifier.insert(style::Modifier::REVERSED);
        }

        Self::default()
            .set_fg(fg)
            .set_bg(bg)
            .set_prefix(value.prefix.clone())
            .insert(modifier)
    }
}
