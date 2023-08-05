pub mod app;

mod app_raw;
mod display_raw;
mod preview_raw;
mod search_raw;
mod sort_raw;
mod tab_raw;

pub use self::app::AppConfig;

#[cfg(not(target_os = "windows"))]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("../../../config/joshuto.toml");

#[cfg(target_os = "windows")]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("..\\..\\..\\config\\joshuto.toml");
