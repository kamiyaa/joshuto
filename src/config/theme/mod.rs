mod app_theme;
mod style;

pub use self::app_theme::AppTheme;
pub use self::style::{AppStyle, RawAppStyle};

#[cfg(not(target_os = "windows"))]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("../../../config/theme.toml");

#[cfg(target_os = "windows")]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("..\\..\\..\\config\\theme.toml");
