mod config;

pub use config::*;

const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("../../../../config/icons.toml");

#[cfg(target_os = "windows")]
const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("..\\..\\..\\..\\config\\icons.toml");
