use serde::Deserialize;
use std::collections::HashMap;

use crate::config::clean::mimetype::ProgramEntry;

#[derive(Clone, Debug, Deserialize)]
pub struct ExtensionAppListRaw {
    #[serde(default, rename = "inherit")]
    pub inherit_class: String,
    #[serde(default)]
    pub app_list: Vec<ProgramEntry>,
}

impl ExtensionAppListRaw {
    pub fn parent(&self) -> &str {
        self.inherit_class.as_str()
    }

    pub fn app_list(&self) -> &[ProgramEntry] {
        self.app_list.as_slice()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct MimetypeAppListRaw {
    #[serde(default, rename = "inherit")]
    pub inherit_class: String,
    #[serde(default)]
    pub app_list: Vec<ProgramEntry>,
    #[serde(default)]
    pub subtype: Option<HashMap<String, ExtensionAppListRaw>>,
}

impl MimetypeAppListRaw {
    pub fn parent(&self) -> &str {
        self.inherit_class.as_str()
    }

    pub fn app_list(&self) -> &[ProgramEntry] {
        self.app_list.as_slice()
    }
}
