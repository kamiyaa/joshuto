use ratatui::style;
use serde::{Deserialize, Serialize};

use crate::utils::style_helper;

/// TOML-deserializable form of [`AppStyle`](super::style::AppStyle).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppStyleRaw {
    #[serde(default)]
    pub fg: String,
    #[serde(default)]
    pub bg: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub invert: bool,
}

impl std::default::Default for AppStyleRaw {
    fn default() -> Self {
        Self {
            bg: "".to_string(),
            fg: "".to_string(),
            prefix: "".to_string(),
            bold: false,
            underline: false,
            invert: false,
        }
    }
}

/// A style override where every field is optional, so it can be patched onto a base style
/// (used for tab-theme prefix/postfix overrides).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppStyleOptionsRaw {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub bold: Option<bool>,
    pub underline: Option<bool>,
    pub invert: Option<bool>,
}

impl From<AppStyleOptionsRaw> for style::Style {
    fn from(value: AppStyleOptionsRaw) -> Self {
        let mut add_modifier = style::Modifier::empty();
        let mut sub_modifier = style::Modifier::empty();
        if let Some(bold) = value.bold {
            if bold {
                add_modifier.insert(style::Modifier::BOLD);
            } else {
                sub_modifier.insert(style::Modifier::BOLD);
            }
        }
        if let Some(underline) = value.underline {
            if underline {
                add_modifier.insert(style::Modifier::UNDERLINED);
            } else {
                sub_modifier.insert(style::Modifier::UNDERLINED);
            }
        }
        if let Some(invert) = value.invert {
            if invert {
                add_modifier.insert(style::Modifier::REVERSED);
            } else {
                sub_modifier.insert(style::Modifier::REVERSED);
            }
        }
        Self {
            fg: value
                .fg
                .clone()
                .map(|s| style_helper::str_to_color(s.as_ref())),
            bg: value
                .bg
                .clone()
                .map(|s| style_helper::str_to_color(s.as_ref())),
            add_modifier,
            sub_modifier,
            //underline_color: None,  # only when ratatui with crossterm
        }
    }
}
