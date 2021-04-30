use serde_derive::Deserialize;

use tui::style::{Color, Modifier};

const fn default_prefix() -> Option<StylePrefix> {
    None
}
const fn default_color() -> Color {
    Color::Reset
}

#[derive(Clone, Debug, Deserialize)]
pub struct StylePrefix {
    prefix: String,
    size: usize,
}

impl StylePrefix {
    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawAppStyle {
    #[serde(default)]
    pub fg: String,
    #[serde(default)]
    pub bg: String,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub invert: bool,
    #[serde(default = "default_prefix")]
    pub prefix: Option<StylePrefix>,
}

impl RawAppStyle {
    pub fn to_style_theme(&self) -> AppStyle {
        let bg = Self::str_to_color(self.bg.as_str());
        let fg = Self::str_to_color(self.fg.as_str());

        let mut modifier = Modifier::empty();
        if self.bold {
            modifier.insert(Modifier::BOLD);
        }
        if self.underline {
            modifier.insert(Modifier::UNDERLINED);
        }
        if self.invert {
            modifier.insert(Modifier::REVERSED);
        }

        AppStyle::default().set_fg(fg).set_bg(bg).insert(modifier)
    }

    pub fn str_to_color(s: &str) -> Color {
        match s {
            "black" => Color::Black,
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "gray" => Color::Gray,
            "dark_gray" => Color::DarkGray,
            "light_red" => Color::LightRed,
            "light_green" => Color::LightGreen,
            "light_yellow" => Color::LightYellow,
            "light_blue" => Color::LightBlue,
            "light_magenta" => Color::LightMagenta,
            "light_cyan" => Color::LightCyan,
            _ => Color::Reset,
        }
    }
}

impl std::default::Default for RawAppStyle {
    fn default() -> Self {
        Self {
            bg: "".to_string(),
            fg: "".to_string(),
            bold: false,
            underline: false,
            invert: false,
            prefix: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppStyle {
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
    pub prefix: Option<StylePrefix>,
}

impl AppStyle {
    pub fn set_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }
    pub fn set_fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }
    pub fn set_prefix(mut self, prefix: StylePrefix) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn insert(mut self, modifier: Modifier) -> Self {
        self.modifier.insert(modifier);
        self
    }
}

impl std::default::Default for AppStyle {
    fn default() -> Self {
        Self {
            fg: default_color(),
            bg: default_color(),
            modifier: Modifier::empty(),
            prefix: None,
        }
    }
}
