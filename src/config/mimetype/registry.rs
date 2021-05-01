use serde_derive::Deserialize;
use std::collections::HashMap;

use super::AppMimetypeEntry;
use crate::config::{parse_config_file, ConfigStructure};

pub type MimetypeRegistry = HashMap<String, Vec<AppMimetypeEntry>>;

#[derive(Debug, Deserialize)]
pub struct AppMimetypeRegistry {
    #[serde(default, skip)]
    empty_vec: Vec<AppMimetypeEntry>,
    #[serde(default)]
    pub extension: MimetypeRegistry,
}

impl AppMimetypeRegistry {
    pub fn get_entries_for_ext(&self, extension: &str) -> &[AppMimetypeEntry] {
        match self.extension.get(extension) {
            Some(s) => s,
            None => &self.empty_vec,
        }
    }
}

impl ConfigStructure for AppMimetypeRegistry {
    fn get_config(file_name: &str) -> Self {
        parse_config_file::<AppMimetypeRegistry>(file_name).unwrap_or_else(Self::default)
    }
}

impl std::default::Default for AppMimetypeRegistry {
    fn default() -> Self {
        Self {
            empty_vec: Vec::new(),
            extension: MimetypeRegistry::new(),
        }
    }
}
