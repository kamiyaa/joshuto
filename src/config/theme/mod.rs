mod app_theme;
mod style;

pub use self::app_theme::*;
pub use self::style::*;

#[cfg(not(target_os = "windows"))]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("../../../config/theme.toml");

#[cfg(target_os = "windows")]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("..\\..\\..\\config\\theme.toml");
