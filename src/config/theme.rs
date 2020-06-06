use serde_derive::Deserialize;
use std::collections::HashMap;

use tui::style::{Color, Modifier};

use crate::THEME_FILE;

use super::{parse_to_config_file, ConfigStructure, Flattenable};

const fn default_prefix() -> Option<JoshutoPrefix> {
    None
}
const fn default_color() -> Color {
    Color::Reset
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoPrefix {
    prefix: String,
    size: usize,
}

impl JoshutoPrefix {
    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoStyleThemeRaw {
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
    pub prefix: Option<JoshutoPrefix>,
}

impl JoshutoStyleThemeRaw {
    pub fn to_style_theme(&self) -> JoshutoStyleTheme {
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

        JoshutoStyleTheme::default()
            .set_fg(fg)
            .set_bg(bg)
            .insert(modifier)
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

impl std::default::Default for JoshutoStyleThemeRaw {
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

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoRawTheme {
    #[serde(default)]
    pub regular: JoshutoStyleThemeRaw,
    #[serde(default)]
    pub selection: JoshutoStyleThemeRaw,
    #[serde(default)]
    pub directory: JoshutoStyleThemeRaw,
    #[serde(default)]
    pub executable: JoshutoStyleThemeRaw,
    #[serde(default)]
    pub link: JoshutoStyleThemeRaw,
    #[serde(default)]
    pub socket: JoshutoStyleThemeRaw,
    #[serde(default)]
    pub ext: HashMap<String, JoshutoStyleThemeRaw>,
}

impl std::default::Default for JoshutoRawTheme {
    fn default() -> Self {
        Self {
            regular: JoshutoStyleThemeRaw::default(),
            selection: JoshutoStyleThemeRaw::default(),
            directory: JoshutoStyleThemeRaw::default(),
            executable: JoshutoStyleThemeRaw::default(),
            link: JoshutoStyleThemeRaw::default(),
            socket: JoshutoStyleThemeRaw::default(),
            ext: HashMap::default(),
        }
    }
}

impl Flattenable<JoshutoTheme> for JoshutoRawTheme {
    fn flatten(self) -> JoshutoTheme {
        let selection = self.selection.to_style_theme();
        let executable = self.executable.to_style_theme();
        let regular = self.regular.to_style_theme();
        let directory = self.directory.to_style_theme();
        let link = self.link.to_style_theme();
        let socket = self.socket.to_style_theme();
        let ext: HashMap<String, JoshutoStyleTheme> = self
            .ext
            .iter()
            .map(|(k, v)| {
                let style = v.to_style_theme();
                (k.clone(), style)
            })
            .collect();

        JoshutoTheme {
            selection,
            executable,
            regular,
            directory,
            link,
            socket,
            ext,
        }
    }
}

#[derive(Clone, Debug)]
pub struct JoshutoStyleTheme {
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
    pub prefix: Option<JoshutoPrefix>,
}

impl JoshutoStyleTheme {
    pub fn set_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }
    pub fn set_fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }
    pub fn set_prefix(mut self, prefix: JoshutoPrefix) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn insert(mut self, modifier: Modifier) -> Self {
        self.modifier.insert(modifier);
        self
    }
}

impl std::default::Default for JoshutoStyleTheme {
    fn default() -> Self {
        Self {
            fg: default_color(),
            bg: default_color(),
            modifier: Modifier::empty(),
            prefix: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct JoshutoTheme {
    pub regular: JoshutoStyleTheme,
    pub selection: JoshutoStyleTheme,
    pub directory: JoshutoStyleTheme,
    pub executable: JoshutoStyleTheme,
    pub link: JoshutoStyleTheme,
    pub socket: JoshutoStyleTheme,
    pub ext: HashMap<String, JoshutoStyleTheme>,
}

impl ConfigStructure for JoshutoTheme {
    fn get_config() -> Self {
        parse_to_config_file::<JoshutoRawTheme, JoshutoTheme>(THEME_FILE)
            .unwrap_or_else(Self::default)
    }
}

impl std::default::Default for JoshutoTheme {
    fn default() -> Self {
        let selection = JoshutoStyleTheme::default()
            .set_fg(Color::LightYellow)
            .insert(Modifier::BOLD);
        let executable = JoshutoStyleTheme::default()
            .set_fg(Color::LightGreen)
            .insert(Modifier::BOLD);
        let regular = JoshutoStyleTheme::default().set_fg(Color::White);
        let directory = JoshutoStyleTheme::default()
            .set_fg(Color::LightBlue)
            .insert(Modifier::BOLD);
        let link = JoshutoStyleTheme::default()
            .set_fg(Color::LightCyan)
            .insert(Modifier::BOLD);
        let socket = JoshutoStyleTheme::default()
            .set_fg(Color::LightMagenta)
            .insert(Modifier::BOLD);
        let ext = HashMap::new();

        Self {
            selection,
            executable,
            regular,
            directory,
            link,
            socket,
            ext,
        }
    }
}
