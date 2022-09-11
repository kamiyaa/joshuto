use serde_derive::Deserialize;
use std::collections::HashMap;

use crate::config::{parse_to_config_file, TomlConfigFile};

use super::{
    ExtensionAppList, ExtensionAppListRaw, MimetypeAppList, MimetypeAppListRaw, ProgramEntry,
};

pub type ExtensionRegistryRaw = HashMap<String, ExtensionAppListRaw>;
pub type MimetypeRegistryRaw = HashMap<String, MimetypeAppListRaw>;

pub type ExtensionRegistry = HashMap<String, ExtensionAppList>;
pub type MimetypeRegistry = HashMap<String, MimetypeAppList>;

#[derive(Debug, Deserialize)]
pub struct AppProgramRegistryRaw {
    #[serde(default, rename = "class")]
    pub _class: HashMap<String, Vec<ProgramEntry>>,
    #[serde(default, rename = "extension")]
    pub _extension: ExtensionRegistryRaw,
    #[serde(default, rename = "mimetype")]
    pub _mimetype: MimetypeRegistryRaw,
}

#[derive(Debug, Default)]
pub struct AppProgramRegistry {
    //    pub _class: HashMap<String, Vec<ProgramEntry>>,
    pub _extension: ExtensionRegistry,
    pub _mimetype: MimetypeRegistry,
}

impl AppProgramRegistry {
    pub fn app_list_for_ext(&self, extension: &str) -> Option<&ExtensionAppList> {
        self._extension.get(extension)
    }

    pub fn app_list_for_mimetype(&self, mimetype: &str) -> Option<&MimetypeAppList> {
        self._mimetype.get(mimetype)
    }
}

impl From<AppProgramRegistryRaw> for AppProgramRegistry {
    fn from(raw: AppProgramRegistryRaw) -> Self {
        let mut extension = ExtensionRegistry::new();
        for (ext, app_list) in raw._extension {
            let class = app_list.parent();
            let mut combined_app_list: ExtensionAppList = raw
                ._class
                .get(class)
                .map(|v| (*v).clone())
                .unwrap_or_default();
            combined_app_list.extend_from_slice(app_list.app_list());

            extension.insert(ext, combined_app_list);
        }

        let mut mimetype = MimetypeRegistry::new();
        for (ttype, data) in raw._mimetype {
            let class = data.parent();
            let mut combined_app_list: ExtensionAppList = raw
                ._class
                .get(class)
                .map(|v| (*v).clone())
                .unwrap_or_default();
            combined_app_list.extend_from_slice(data.app_list());

            let subtypes = data
                .subtype
                .unwrap_or_default()
                .into_iter()
                .map(|(k, v)| (k, v.app_list))
                .collect();
            let app_list = MimetypeAppList::new(combined_app_list, subtypes);
            mimetype.insert(ttype, app_list);
        }

        Self {
            _extension: extension,
            _mimetype: mimetype,
        }
    }
}

impl TomlConfigFile for AppProgramRegistry {
    fn get_config(file_name: &str) -> Self {
        match parse_to_config_file::<AppProgramRegistryRaw, AppProgramRegistry>(file_name) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to parse mimetype config: {}", e);
                Self::default()
            }
        }
    }
}
