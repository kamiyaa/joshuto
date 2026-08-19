//! Default config file contents, embedded into the binary at compile time.

/// Default `joshuto.toml` contents, used when no user config is found.
pub const APP_CONFIG: &str = include_str!("../../config/joshuto.toml");
/// Default `icons.toml` contents, used when no user config is found.
pub const ICON_CONFIG: &str = include_str!("../../config/icons.toml");
/// Default `keymap.toml` contents, used when no user config is found.
pub const KEYMAP_CONFIG: &str = include_str!("../../config/keymap.toml");
/// Default `theme.toml` contents, used when no user config is found.
pub const THEME_CONFIG: &str = include_str!("../../config/theme.toml");
