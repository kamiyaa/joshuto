use std::collections::HashMap;

use super::ProgramEntry;

/// The configured programs to open a file with, for one extension.
pub type ExtensionAppList = Vec<ProgramEntry>;

/// The configured programs to open a file with, for one MIME type, plus any per-subtype
/// overrides (e.g. `text/plain` vs. `text/*`).
#[derive(Clone, Debug)]
pub struct MimetypeAppList {
    pub app_list: Vec<ProgramEntry>,
    pub subtypes: HashMap<String, ExtensionAppList>,
}

impl MimetypeAppList {
    /// Builds a `MimetypeAppList` from a resolved program list and subtype overrides.
    pub fn new(app_list: Vec<ProgramEntry>, subtypes: HashMap<String, ExtensionAppList>) -> Self {
        Self { app_list, subtypes }
    }
}
