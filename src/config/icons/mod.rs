use std::collections::HashMap;

use crate::error::JoshutoResult;

pub use self::icons_raw::*;

use super::{parse_config_or_default, TomlConfigFile};

#[cfg(not(target_os = "windows"))]
const DEFAULT_ICONS: &str = include_str!("../../../config/icons.toml");

#[cfg(target_os = "windows")]
const DEFAULT_ICONS: &str = include_str!("..\\..\\..\\config\\icons.toml");

pub mod icons_raw;

#[derive(Debug)]
pub struct Icons {
    pub directory_exact: HashMap<String, String>,
    pub file_exact: HashMap<String, String>,
    pub ext: HashMap<String, String>,
    pub default_file: String,
    pub default_dir: String,
}

impl From<IconsRaw> for Icons {
    fn from(value: IconsRaw) -> Self {
        Icons {
            directory_exact: value.directory_exact,
            file_exact: value.file_exact,
            ext: value.ext,
            default_file: value.defaults.file,
            default_dir: value.defaults.directory,
        }
    }
}

impl Icons {
    pub(crate) fn default_icons() -> JoshutoResult<Self> {
        let icons: IconsRaw = toml::from_str(DEFAULT_ICONS)?;
        Ok(Self::from(icons))
    }
}

impl std::default::Default for Icons {
    fn default() -> Self {
        Self::default_icons().unwrap()
    }
}

impl TomlConfigFile for Icons {
    fn get_config(file_name: &str) -> Self {
        parse_config_or_default::<IconsRaw, Icons>(file_name)
    }
}
