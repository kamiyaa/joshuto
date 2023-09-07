use std::collections::HashMap;

use super::ProgramEntry;

pub type ExtensionAppList = Vec<ProgramEntry>;

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
