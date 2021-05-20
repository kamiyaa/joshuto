use serde_derive::Deserialize;
use std::collections::HashMap;

use super::{AppList, AppMimetypeEntry};
use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};

pub type MimetypeRegistry = HashMap<String, AppList>;

#[derive(Debug, Deserialize)]
pub struct RawAppMimetypeRegistry {
    #[serde(default, rename = "class")]
    pub _class: HashMap<String, Vec<AppMimetypeEntry>>,
    #[serde(default, rename = "extension")]
    pub _extension: MimetypeRegistry,
}

impl Flattenable<AppMimetypeRegistry> for RawAppMimetypeRegistry {
    fn flatten(self) -> AppMimetypeRegistry {
        let mut registry = MimetypeRegistry::new();

        for (ext, app_list) in self._extension {
            let class = app_list.parent();
            let mut combined_app_list: Vec<AppMimetypeEntry> = self
                ._class
                .get(class)
                .map(|v| (*v).clone())
                .unwrap_or_default();
            combined_app_list.extend_from_slice(app_list.app_list());
            let combined_app_list = AppList::new(class.to_string(), combined_app_list);
            registry.insert(ext, combined_app_list);
        }

        AppMimetypeRegistry {
            _extension: registry,
        }
    }
}

#[derive(Debug)]
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

impl ConfigStructure for AppMimetypeRegistry {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<RawAppMimetypeRegistry, AppMimetypeRegistry>(file_name)
            .unwrap_or_else(Self::default)
    }
}

impl std::default::Default for AppMimetypeRegistry {
    fn default() -> Self {
        Self {
            _extension: MimetypeRegistry::new(),
        }
    }
}
