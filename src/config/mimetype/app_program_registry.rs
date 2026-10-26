use std::collections::HashMap;

use crate::config::{
    mimetype::{application_list_raw::AppProgramRegistryRaw, ExtensionAppList, MimetypeAppList},
    AppPersistentConfig, ConfigType,
};

/// Programs to open a file with, keyed by file extension.
pub type ExtensionRegistry = HashMap<String, ExtensionAppList>;
/// Programs to open a file with, keyed by MIME type.
pub type MimetypeRegistry = HashMap<String, MimetypeAppList>;

/// Resolved `open_with` program lists for every configured file extension and MIME type, with
/// class-based inheritance already flattened in.
#[derive(Debug, Default)]
pub struct AppProgramRegistry {
    //    pub _class: HashMap<String, Vec<ProgramEntry>>,
    pub extension: ExtensionRegistry,
    pub mimetype: MimetypeRegistry,
}

impl AppProgramRegistry {
    /// Returns the configured programs to open files with extension `extension`.
    pub fn app_list_for_ext(&self, extension: &str) -> Option<&ExtensionAppList> {
        self.extension.get(extension)
    }

    /// Returns the configured programs to open files of MIME type `mimetype`.
    pub fn app_list_for_mimetype(&self, mimetype: &str) -> Option<&MimetypeAppList> {
        self.mimetype.get(mimetype)
    }
}

impl From<AppProgramRegistryRaw> for AppProgramRegistry {
    fn from(value: AppProgramRegistryRaw) -> Self {
        let mut extension = ExtensionRegistry::new();
        for (ext, app_list) in value._extension {
            let class = app_list.parent();
            let mut combined_app_list: ExtensionAppList = value
                ._class
                .get(class)
                .map(|v| (*v).clone())
                .unwrap_or_default();
            combined_app_list.extend_from_slice(app_list.app_list());

            extension.insert(ext, combined_app_list);
        }

        let mut mimetype = MimetypeRegistry::new();
        for (ttype, data) in value._mimetype {
            let class = data.parent();
            let mut combined_app_list: ExtensionAppList = value
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

impl AppPersistentConfig for AppProgramRegistry {
    type Raw = AppProgramRegistryRaw;

    fn get_type() -> ConfigType {
        ConfigType::Mimetype
    }
}
