use serde::Deserialize;
use std::collections::HashMap;

use crate::types::mimetype::ProgramEntry;

/// TOML-deserializable form of [`ExtensionRegistry`](super::mimetype::ExtensionRegistry).
pub type ExtensionRegistryRaw = HashMap<String, ExtensionAppListRaw>;
/// TOML-deserializable form of [`MimetypeRegistry`](super::mimetype::MimetypeRegistry).
pub type MimetypeRegistryRaw = HashMap<String, MimetypeAppListRaw>;

/// Configured `open_with` programs for one file extension, before class inheritance is resolved.
#[derive(Clone, Debug, Deserialize)]
pub struct ExtensionAppListRaw {
    #[serde(default, rename = "inherit")]
    pub inherit_class: String,
    #[serde(default)]
    pub app_list: Vec<ProgramEntry>,
}

impl ExtensionAppListRaw {
    /// Returns the name of the class this extension inherits programs from, if any.
    pub fn parent(&self) -> &str {
        self.inherit_class.as_str()
    }

    /// Returns this extension's own configured programs, excluding inherited ones.
    pub fn app_list(&self) -> &[ProgramEntry] {
        self.app_list.as_slice()
    }
}

/// Configured `open_with` programs for one MIME type, before class inheritance is resolved.
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
    /// Returns the name of the class this MIME type inherits programs from, if any.
    pub fn parent(&self) -> &str {
        self.inherit_class.as_str()
    }

    /// Returns this MIME type's own configured programs, excluding inherited ones.
    pub fn app_list(&self) -> &[ProgramEntry] {
        self.app_list.as_slice()
    }
}

/// TOML-deserializable form of `mimetype.toml`, before class inheritance is resolved into an
/// [`AppProgramRegistry`](super::mimetype::AppProgramRegistry).
#[derive(Debug, Deserialize)]
pub struct AppProgramRegistryRaw {
    #[serde(default, rename = "class")]
    pub _class: HashMap<String, Vec<ProgramEntry>>,
    #[serde(default, rename = "extension")]
    pub _extension: ExtensionRegistryRaw,
    #[serde(default, rename = "mimetype")]
    pub _mimetype: MimetypeRegistryRaw,
}
