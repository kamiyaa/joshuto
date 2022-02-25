use serde_derive::Deserialize;
use std::collections::HashMap;

use crate::config::{parse_to_config_file, TomlConfigFile};

use super::{AppList, AppMimetypeEntry};

pub type MimetypeRegistry = HashMap<String, AppList>;

#[derive(Debug, Deserialize)]
pub struct AppMimetypeRegistryCrude {
    #[serde(default, rename = "class")]
    pub _class: HashMap<String, Vec<AppMimetypeEntry>>,
    #[serde(default, rename = "extension")]
    pub _extension: MimetypeRegistry,
}

#[derive(Debug, Default)]
pub struct AppMimetypeRegistry {
    //    pub _class: HashMap<String, Vec<AppMimetypeEntry>>,
    pub _extension: MimetypeRegistry,
}

pub const EMPTY_ARR: [AppMimetypeEntry; 0] = [];

impl AppMimetypeRegistry {
    pub fn app_list_for_ext(&self, extension: &str) -> &[AppMimetypeEntry] {
        match self._extension.get(extension) {
            Some(s) => s.app_list(),
            None => &EMPTY_ARR,
        }
    }
}

impl From<AppMimetypeRegistryCrude> for AppMimetypeRegistry {
    fn from(crude: AppMimetypeRegistryCrude) -> Self {
        let mut registry = MimetypeRegistry::new();

        for (ext, app_list) in crude._extension {
            let class = app_list.parent();
            let mut combined_app_list: Vec<AppMimetypeEntry> = crude
                ._class
                .get(class)
                .map(|v| (*v).clone())
                .unwrap_or_default();
            combined_app_list.extend_from_slice(app_list.app_list());
            let combined_app_list = AppList::new(class.to_string(), combined_app_list);
            registry.insert(ext, combined_app_list);
        }

        Self {
            _extension: registry,
        }
    }
}

impl TomlConfigFile for AppMimetypeRegistry {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<AppMimetypeRegistryCrude, AppMimetypeRegistry>(file_name)
            .unwrap_or_default()
    }
}
