use serde::de::DeserializeOwned;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::AppResult;
use crate::types::config_type::ConfigType;
use crate::CONFIG_HIERARCHY;

/// Implemented by types that are loaded from a TOML config file, with a `Raw` deserialization
/// form and a default used when no file is found or parsing fails.
pub trait TomlConfigFile: Sized + Default {
    /// The `serde`-deserializable form read from disk, convertible into `Self`.
    type Raw: Into<Self> + DeserializeOwned;

    /// Returns which config file this type is loaded from.
    fn get_type() -> ConfigType;

    /// Loads and parses this config, falling back to `Self::default()` if not found or invalid.
    fn get_config() -> Self {
        parse_config_or_default::<Self::Raw, Self>(Self::get_type().as_filename())
    }
}

/// Searches `directories` in order for a file named `file_name`, returning the first match.
pub fn search_directories<P>(file_name: &str, directories: &[P]) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    directories
        .iter()
        .map(|path| path.as_ref().join(file_name))
        .find(|path| path.exists())
}

/// Searches joshuto's config directory hierarchy for a file named `file_name`.
pub fn search_config_directories(file_name: &str) -> Option<PathBuf> {
    search_directories(file_name, &CONFIG_HIERARCHY)
}

fn parse_file_to_config<T, S>(file_path: &Path) -> AppResult<S>
where
    T: DeserializeOwned + Into<S>,
{
    let file_contents = fs::read_to_string(file_path)?;
    let config = toml::from_str::<T>(&file_contents)?;
    Ok(config.into())
}

/// Finds, reads, and parses `file_name` from the config directory hierarchy, returning
/// `S::default()` (and printing a warning) if it's missing or fails to parse.
pub fn parse_config_or_default<T, S>(file_name: &str) -> S
where
    T: DeserializeOwned + Into<S>,
    S: std::default::Default,
{
    match search_config_directories(file_name) {
        Some(file_path) => match parse_file_to_config::<T, S>(&file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to parse {}: {}", file_name, e);
                S::default()
            }
        },
        None => S::default(),
    }
}
