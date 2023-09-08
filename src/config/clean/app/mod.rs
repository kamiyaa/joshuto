pub mod config;
pub mod display;
pub mod preview;
pub mod search;
pub mod tab;

pub use config::*;

#[cfg(not(target_os = "windows"))]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("../../../../config/joshuto.toml");

#[cfg(target_os = "windows")]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("..\\..\\..\\..\\config\\joshuto.toml");
