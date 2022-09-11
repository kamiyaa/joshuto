use serde_derive::Deserialize;
use std::collections::HashMap;

use super::ProgramEntry;

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

pub type ExtensionAppList = Vec<ProgramEntry>;

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

#[derive(Clone, Debug)]
pub struct MimetypeAppList {
    _app_list: Vec<ProgramEntry>,
    _subtypes: HashMap<String, ExtensionAppList>,
}

impl MimetypeAppList {
    pub fn new(_app_list: Vec<ProgramEntry>, _subtypes: HashMap<String, ExtensionAppList>) -> Self {
        Self {
            _app_list,
            _subtypes,
        }
    }
    pub fn app_list(&self) -> &[ProgramEntry] {
        self._app_list.as_slice()
    }

    pub fn subtypes(&self) -> &HashMap<String, ExtensionAppList> {
        &self._subtypes
    }
}
