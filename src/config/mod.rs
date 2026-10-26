mod app;
mod app_icon;
mod app_icon_raw;
mod app_raw;
mod bookmarks;
mod config_type;
mod custom_command;
mod display;
mod file_entry_preview;
mod keymapping;
mod mimetype;
mod theme;

pub(crate) mod utils;

use serde::de::DeserializeOwned;

pub use app::*;
pub use app_icon::*;
pub use bookmarks::*;
pub use config_type::*;
pub use custom_command::*;
pub use display::*;
pub use file_entry_preview::*;
pub use keymapping::*;
pub use mimetype::*;
pub use theme::*;

use crate::config::utils::parse_config_or_default;

/// Implemented by types that are loaded from a TOML config file, with a `Raw` deserialization
/// form and a default used when no file is found or parsing fails.
pub trait AppPersistentConfig: Sized + Default {
    /// The `serde`-deserializable form read from disk, convertible into `Self`.
    type Raw: Into<Self> + DeserializeOwned;

    /// Returns which config file this type is loaded from.
    fn get_type() -> ConfigType;

    /// Loads and parses this config, falling back to `Self::default()` if not found or invalid.
    fn read_config() -> Self {
        parse_config_or_default::<Self::Raw, Self>(Self::get_type().as_filename())
    }
}
