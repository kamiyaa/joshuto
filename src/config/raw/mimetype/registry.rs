use serde::Deserialize;
use std::collections::HashMap;

use crate::config::clean::mimetype::ProgramEntry;

use super::{ExtensionAppListRaw, MimetypeAppListRaw};

pub type ExtensionRegistryRaw = HashMap<String, ExtensionAppListRaw>;
pub type MimetypeRegistryRaw = HashMap<String, MimetypeAppListRaw>;

#[derive(Debug, Deserialize)]
pub struct AppProgramRegistryRaw {
    #[serde(default, rename = "class")]
    pub _class: HashMap<String, Vec<ProgramEntry>>,
    #[serde(default, rename = "extension")]
    pub _extension: ExtensionRegistryRaw,
    #[serde(default, rename = "mimetype")]
    pub _mimetype: MimetypeRegistryRaw,
}
