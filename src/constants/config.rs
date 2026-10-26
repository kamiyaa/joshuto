//! Default config file contents, embedded into the binary at compile time.

/// Default `joshuto.toml` contents, used when no user config is found.
pub const DEFAULT_APP_CONFIG: &str = include_str!("../../config/joshuto.toml");
/// Default `icons.toml` contents, used when no user config is found.
pub const DEFAULT_ICON_CONFIG: &str = include_str!("../../config/icons.toml");
/// Default `keymap.toml` contents, used when no user config is found.
pub const DEFAULT_KEYMAP_CONFIG: &str = include_str!("../../config/keymap.toml");
/// Default `theme.toml` contents, used when no user config is found.
pub const DEFAULT_THEME_CONFIG: &str = include_str!("../../config/theme.toml");
