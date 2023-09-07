mod config;

pub use self::config::*;

#[cfg(not(target_os = "windows"))]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("../../../../config/keymap.toml");

#[cfg(target_os = "windows")]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("..\\..\\..\\..\\config\\keymap.toml");
