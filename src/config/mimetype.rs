use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt;

use super::{parse_config_file, ConfigStructure, Flattenable};
use crate::MIMETYPE_FILE;

const fn default_false() -> bool { false }

#[derive(Debug, Deserialize)]
pub struct JoshutoMimetypeEntry {
    pub id: usize,
    pub program: String,
    pub args: Option<Vec<String>>,
    #[serde(default = "default_false")]
    pub fork: bool,
    #[serde(default = "default_false")]
    pub silent: bool,
}

impl std::fmt::Display for JoshutoMimetypeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.program.as_str()).unwrap();
        if let Some(s) = self.args.as_ref() {
            s.iter().for_each(|arg| write!(f, " {}", arg).unwrap());
        }
        f.write_str("\t[").unwrap();
        if self.fork {
            f.write_str("fork,").unwrap();
        }
        if self.silent {
            f.write_str("silent").unwrap();
        }
        f.write_str("]")
    }
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawMimetype {
    #[serde(default)]
    entry: Vec<JoshutoMimetypeEntry>,
    #[serde(default)]
    extension: HashMap<String, Vec<usize>>,
    #[serde(default)]
    mimetype: HashMap<String, Vec<usize>>,
}

impl Flattenable<JoshutoMimetype> for JoshutoRawMimetype {
    fn flatten(self) -> JoshutoMimetype {
        let mut entries = HashMap::with_capacity(self.entry.len());
        for entry in self.entry {
            entries.insert(entry.id, entry);
        }
        JoshutoMimetype {
            entries,
            extension: self.extension,
            mimetype: self.mimetype,
        }
    }
}

#[derive(Debug)]
pub struct JoshutoMimetype {
    pub entries: HashMap<usize, JoshutoMimetypeEntry>,
    pub extension: HashMap<String, Vec<usize>>,
    pub mimetype: HashMap<String, Vec<usize>>,
}

impl JoshutoMimetype {
    pub fn get_entries_for_ext(&self, extension: &str) -> Vec<&JoshutoMimetypeEntry> {
        let mut vec = Vec::new();
        if let Some(entry_ids) = self.extension.get(extension) {
            for id in entry_ids {
                if let Some(s) = self.entries.get(id) {
                    vec.push(s);
                }
            }
        }
        vec
    }
    pub fn get_entries_for_mimetype(&self, mimetype: &str) -> Vec<&JoshutoMimetypeEntry> {
        let mut vec = Vec::new();
        if let Some(entry_ids) = self.mimetype.get(mimetype) {
            for id in entry_ids {
                if let Some(s) = self.entries.get(id) {
                    vec.push(s);
                }
            }
        }
        vec
    }
}

impl ConfigStructure for JoshutoMimetype {
    fn get_config() -> Self {
        parse_config_file::<JoshutoRawMimetype, JoshutoMimetype>(MIMETYPE_FILE)
            .unwrap_or_else(JoshutoMimetype::default)
    }
}

impl std::default::Default for JoshutoMimetype {
    fn default() -> Self {
        JoshutoMimetype {
            entries: HashMap::new(),
            mimetype: HashMap::new(),
            extension: HashMap::new(),
        }
    }
}
