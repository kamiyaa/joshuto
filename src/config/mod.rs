pub mod general;
pub mod keymap;
pub mod mimetype;
pub mod option;
pub mod preview;
pub mod theme;

pub use self::general::AppConfig;
pub use self::keymap::*;
pub use self::mimetype::*;
pub use self::preview::*;
pub use self::theme::*;

use serde::de::DeserializeOwned;
use std::fs;
use std::path::{Path, PathBuf};

use crate::CONFIG_HIERARCHY;

pub trait TomlConfigFile {
    fn get_config(file_name: &str) -> Self;
}

// searches a list of folders for a given file in order of preference
pub fn search_directories<P>(filename: &str, directories: &[P]) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    for path in directories.iter() {
        let filepath = path.as_ref().join(filename);
        if filepath.exists() {
            return Some(filepath);
        }
    }
    None
}

// parses a config file into its appropriate format
fn parse_to_config_file<T, S>(filename: &str) -> Option<S>
where
    T: DeserializeOwned,
    S: From<T>,
{
    let file_path = search_directories(filename, &CONFIG_HIERARCHY)?;
    let file_contents = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading {} file: {}", filename, e);
            return None;
        }
    };
    let config = match toml::from_str::<T>(&file_contents) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing {} file: {}", filename, e);
            return None;
        }
    };
    Some(S::from(config))
}
