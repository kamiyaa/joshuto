use std::collections::HashMap;

use crate::{
    config::{raw::icon::IconsRaw, ConfigType, TomlConfigFile},
    error::AppResult,
};

use super::DEFAULT_CONFIG_FILE_PATH;

#[derive(Debug)]
pub struct Icons {
    pub directory_exact: HashMap<String, String>,
    pub file_exact: HashMap<String, String>,
    pub ext: HashMap<String, String>,
    pub default_file: String,
    pub default_dir: String,
}

impl Icons {
    pub fn default_icons() -> AppResult<Self> {
        let icons: IconsRaw = toml::from_str(DEFAULT_CONFIG_FILE_PATH)?;
        Ok(Self::from(icons))
    }
}

impl std::default::Default for Icons {
    fn default() -> Self {
        Self::default_icons().unwrap()
    }
}

impl TomlConfigFile for Icons {
    type Raw = IconsRaw;

    fn get_type() -> ConfigType {
        ConfigType::Icons
    }
}

impl From<IconsRaw> for Icons {
    fn from(raw: IconsRaw) -> Self {
        Icons {
            directory_exact: raw.directory_exact,
            file_exact: raw.file_exact,
            ext: raw.ext,
            default_file: raw.defaults.file,
            default_dir: raw.defaults.directory,
        }
    }
}
