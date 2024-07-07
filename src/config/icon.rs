use std::collections::HashMap;

use crate::{
    constants::config::ICON_CONFIG, error::AppResult, traits::config::TomlConfigFile,
    types::config_type::ConfigType,
};

use super::icon_raw::IconsRaw;

#[derive(Debug)]
pub struct AppIcons {
    pub directory_exact: HashMap<String, String>,
    pub file_exact: HashMap<String, String>,
    pub ext: HashMap<String, String>,
    pub default_file: String,
    pub default_dir: String,
}

impl AppIcons {
    pub fn default_icons() -> AppResult<Self> {
        let icons: IconsRaw = toml::from_str(ICON_CONFIG)?;
        Ok(Self::from(icons))
    }
}

impl std::default::Default for AppIcons {
    fn default() -> Self {
        Self::default_icons().unwrap()
    }
}

impl TomlConfigFile for AppIcons {
    type Raw = IconsRaw;

    fn get_type() -> ConfigType {
        ConfigType::Icons
    }
}

impl From<IconsRaw> for AppIcons {
    fn from(raw: IconsRaw) -> Self {
        AppIcons {
            directory_exact: raw.directory_exact,
            file_exact: raw.file_exact,
            ext: raw.ext,
            default_file: raw.defaults.file,
            default_dir: raw.defaults.directory,
        }
    }
}
