use serde_derive::Deserialize;
use std::collections::HashMap;

use tui::style::{Color, Modifier};

use super::{AppStyle, RawAppStyle};
use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};

const fn default_color() -> Color {
    Color::Reset
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawAppTheme {
    #[serde(default)]
    pub regular: RawAppStyle,
    #[serde(default)]
    pub selection: RawAppStyle,
    #[serde(default)]
    pub directory: RawAppStyle,
    #[serde(default)]
    pub executable: RawAppStyle,
    #[serde(default)]
    pub link: RawAppStyle,
    #[serde(default)]
    pub socket: RawAppStyle,
    #[serde(default)]
    pub ext: HashMap<String, RawAppStyle>,
}

impl std::default::Default for RawAppTheme {
    fn default() -> Self {
        Self {
            regular: RawAppStyle::default(),
            selection: RawAppStyle::default(),
            directory: RawAppStyle::default(),
            executable: RawAppStyle::default(),
            link: RawAppStyle::default(),
            socket: RawAppStyle::default(),
            ext: HashMap::default(),
        }
    }
}

impl Flattenable<AppTheme> for RawAppTheme {
    fn flatten(self) -> AppTheme {
        let selection = self.selection.to_style_theme();
        let executable = self.executable.to_style_theme();
        let regular = self.regular.to_style_theme();
        let directory = self.directory.to_style_theme();
        let link = self.link.to_style_theme();
        let socket = self.socket.to_style_theme();
        let ext: HashMap<String, AppStyle> = self
            .ext
            .iter()
            .map(|(k, v)| {
                let style = v.to_style_theme();
                (k.clone(), style)
            })
            .collect();

        AppTheme {
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
pub struct AppTheme {
    pub regular: AppStyle,
    pub selection: AppStyle,
    pub directory: AppStyle,
    pub executable: AppStyle,
    pub link: AppStyle,
    pub socket: AppStyle,
    pub ext: HashMap<String, AppStyle>,
}

impl ConfigStructure for AppTheme {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<RawAppTheme, AppTheme>(file_name).unwrap_or_else(Self::default)
    }
}

impl std::default::Default for AppTheme {
    fn default() -> Self {
        let selection = AppStyle::default()
            .set_fg(Color::LightYellow)
            .insert(Modifier::BOLD);
        let executable = AppStyle::default()
            .set_fg(Color::LightGreen)
            .insert(Modifier::BOLD);
        let regular = AppStyle::default().set_fg(Color::White);
        let directory = AppStyle::default()
            .set_fg(Color::LightBlue)
            .insert(Modifier::BOLD);
        let link = AppStyle::default()
            .set_fg(Color::LightCyan)
            .insert(Modifier::BOLD);
        let socket = AppStyle::default()
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
