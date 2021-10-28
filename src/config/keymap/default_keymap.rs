#[cfg(not(target_os = "windows"))]
pub const DEFAULT_KEYMAP: &str = include_str!("../../../config/keymap.toml");

#[cfg(target_os = "windows")]
pub const DEFAULT_KEYMAP: &str = include_str!("..\\..\\..\\config\\keymap.toml");
