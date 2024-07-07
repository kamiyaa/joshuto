use std::collections::HashMap;

use crate::{
    traits::config::TomlConfigFile,
    types::{
        config_type::ConfigType,
        mimetype::{ExtensionAppList, MimetypeAppList},
    },
};

use super::mimetype_raw::AppProgramRegistryRaw;

pub type ExtensionRegistry = HashMap<String, ExtensionAppList>;
pub type MimetypeRegistry = HashMap<String, MimetypeAppList>;

#[derive(Debug, Default)]
pub struct AppProgramRegistry {
    //    pub _class: HashMap<String, Vec<ProgramEntry>>,
    pub extension: ExtensionRegistry,
    pub mimetype: MimetypeRegistry,
}

impl AppProgramRegistry {
    pub fn app_list_for_ext(&self, extension: &str) -> Option<&ExtensionAppList> {
        self.extension.get(extension)
    }

    pub fn app_list_for_mimetype(&self, mimetype: &str) -> Option<&MimetypeAppList> {
        self.mimetype.get(mimetype)
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
            extension,
            mimetype,
        }
    }
}

impl TomlConfigFile for AppProgramRegistry {
    type Raw = AppProgramRegistryRaw;

    fn get_type() -> ConfigType {
        ConfigType::Mimetype
    }
}
