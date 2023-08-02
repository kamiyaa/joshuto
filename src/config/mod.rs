pub mod bookmarks;
pub mod general;
pub mod icons;
pub mod keymap;
pub mod mimetype;
pub mod option;
pub mod preview;
pub mod theme;

pub use self::bookmarks::*;
pub use self::general::*;
pub use self::keymap::*;
pub use self::mimetype::*;
pub use self::preview::*;
pub use self::theme::*;

use serde::de::DeserializeOwned;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::JoshutoResult;
use crate::CONFIG_HIERARCHY;

pub trait TomlConfigFile {
    fn get_config(file_name: &str) -> Self;
}

// searches a list of folders for a given file in order of preference
pub fn search_directories<P>(file_name: &str, directories: &[P]) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    for path in directories.iter() {
        let file_path = path.as_ref().join(file_name);
        if file_path.exists() {
            return Some(file_path);
        }
    }
    None
}

pub fn search_config_directories(file_name: &str) -> Option<PathBuf> {
    search_directories(file_name, &CONFIG_HIERARCHY)
}

fn parse_file_to_config<T, S>(file_path: &Path) -> JoshutoResult<S>
where
    T: DeserializeOwned,
    S: From<T>,
{
    let file_contents = fs::read_to_string(file_path)?;
    let config = toml::from_str::<T>(&file_contents)?;
    Ok(S::from(config))
}

pub fn parse_config_or_default<T, S>(file_name: &str) -> S
where
    T: DeserializeOwned,
    S: From<T> + std::default::Default,
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
