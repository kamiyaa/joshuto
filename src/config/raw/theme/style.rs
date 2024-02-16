use colors_transform::{Color, Rgb};
use ratatui::style::{self, Style};
use serde::Deserialize;

use crate::config::clean::theme::style::AppStyle;

fn str_to_color(s: &str) -> style::Color {
    match s {
        "black" => style::Color::Black,
        "red" => style::Color::Red,
        "green" => style::Color::Green,
        "yellow" => style::Color::Yellow,
        "blue" => style::Color::Blue,
        "magenta" => style::Color::Magenta,
        "cyan" => style::Color::Cyan,
        "gray" => style::Color::Gray,
        "dark_gray" => style::Color::DarkGray,
        "light_red" => style::Color::LightRed,
        "light_green" => style::Color::LightGreen,
        "light_yellow" => style::Color::LightYellow,
        "light_blue" => style::Color::LightBlue,
        "light_magenta" => style::Color::LightMagenta,
        "light_cyan" => style::Color::LightCyan,
        "white" => style::Color::White,
        "reset" => style::Color::Reset,
        s if s.starts_with('#') => {
            let rgb = match Rgb::from_hex_str(s) {
                Ok(s) => s,
                _ => return style::Color::Reset,
            };
            let r = rgb.get_red() as u8;
            let g = rgb.get_green() as u8;
            let b = rgb.get_blue() as u8;
            style::Color::Rgb(r, g, b)
        }
        s if s.is_empty() => style::Color::Reset,
        s => match s.parse::<Rgb>() {
            Ok(rgb) => {
                let r = rgb.get_red() as u8;
                let g = rgb.get_green() as u8;
                let b = rgb.get_blue() as u8;
                style::Color::Rgb(r, g, b)
            }
            Err(_) => style::Color::Reset,
        },
    }
}

#[derive(Clone, Debug, Deserialize)]

pub struct AppStyleOptionsRaw {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub bold: Option<bool>,
    pub underline: Option<bool>,
    pub invert: Option<bool>,
}

impl AppStyleOptionsRaw {
    pub fn as_style(&self) -> Style {
        let mut add_modifier = style::Modifier::empty();
        let mut sub_modifier = style::Modifier::empty();
        if let Some(bold) = self.bold {
            if bold {
                add_modifier.insert(style::Modifier::BOLD);
            } else {
                sub_modifier.insert(style::Modifier::BOLD);
            }
        }
        if let Some(underline) = self.underline {
            if underline {
                add_modifier.insert(style::Modifier::UNDERLINED);
            } else {
                sub_modifier.insert(style::Modifier::UNDERLINED);
            }
        }
        if let Some(invert) = self.invert {
            if invert {
                add_modifier.insert(style::Modifier::REVERSED);
            } else {
                sub_modifier.insert(style::Modifier::REVERSED);
            }
        }
        Style {
            fg: self.fg.clone().map(|s| str_to_color(s.as_ref())),
            bg: self.bg.clone().map(|s| str_to_color(s.as_ref())),
            add_modifier,
            sub_modifier,
            //underline_color: None,  # only when ratatui with crossterm
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppStyleRaw {
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
}

impl AppStyleRaw {
    pub fn to_style_theme(&self) -> AppStyle {
        let bg = Self::str_to_color(self.bg.as_str());
        let fg = Self::str_to_color(self.fg.as_str());

        let mut modifier = style::Modifier::empty();
        if self.bold {
            modifier.insert(style::Modifier::BOLD);
        }
        if self.underline {
            modifier.insert(style::Modifier::UNDERLINED);
        }
        if self.invert {
            modifier.insert(style::Modifier::REVERSED);
        }

        AppStyle::default().set_fg(fg).set_bg(bg).insert(modifier)
    }

    pub fn str_to_color(s: &str) -> style::Color {
        str_to_color(s)
    }
}

impl std::default::Default for AppStyleRaw {
    fn default() -> Self {
        Self {
            bg: "".to_string(),
            fg: "".to_string(),
            bold: false,
            underline: false,
            invert: false,
        }
    }
}
