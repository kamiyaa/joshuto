use std::collections::HashMap;

use super::ProgramEntry;

pub type ExtensionAppList = Vec<ProgramEntry>;

#[derive(Clone, Debug)]
pub struct MimetypeAppList {
    pub app_list: Vec<ProgramEntry>,
    pub subtypes: HashMap<String, ExtensionAppList>,
}

impl MimetypeAppList {
    pub fn new(app_list: Vec<ProgramEntry>, subtypes: HashMap<String, ExtensionAppList>) -> Self {
        Self { app_list, subtypes }
    }
}
