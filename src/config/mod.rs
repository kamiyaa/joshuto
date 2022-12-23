pub mod bookmarks;
pub mod general;
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
use std::io;
use std::path::{Path, PathBuf};

use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
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
fn parse_to_config_file<T, S>(filename: &str) -> JoshutoResult<S>
where
    T: DeserializeOwned,
    S: From<T>,
{
    match search_directories(filename, &CONFIG_HIERARCHY) {
        Some(file_path) => {
            let file_contents = fs::read_to_string(&file_path)?;
            let config = toml::from_str::<T>(&file_contents)?;
            Ok(S::from(config))
        }
        None => {
            let error_kind = io::ErrorKind::NotFound;
            let error = JoshutoError::new(
                JoshutoErrorKind::Io(error_kind),
                format!("Configuration file {} not found", filename),
            );
            Err(error)
        }
    }
}
