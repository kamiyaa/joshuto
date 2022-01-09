pub mod app;

mod app_crude;
mod display_crude;
mod preview_crude;
mod sort_crude;
mod tab_crude;

pub use self::app::AppConfig;

#[cfg(not(target_os = "windows"))]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("../../../config/joshuto.toml");

#[cfg(target_os = "windows")]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("..\\..\\..\\config\\joshuto.toml");
