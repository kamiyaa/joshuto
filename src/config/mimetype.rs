use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt;

use crate::config::{parse_config_file, Flattenable};
use crate::MIMETYPE_FILE;

#[derive(Debug, Deserialize)]
pub struct JoshutoMimetypeEntry {
    pub id: usize,
    pub program: String,
    pub args: Option<Vec<String>>,
    pub fork: Option<bool>,
    pub silent: Option<bool>,
}

impl std::fmt::Display for JoshutoMimetypeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.program.as_str()).unwrap();
        if let Some(s) = self.args.as_ref() {
            s.iter().for_each(|arg| write!(f, " {}", arg).unwrap());
        }
        f.write_str("\t[").unwrap();
        if let Some(s) = self.fork {
            if s {
                f.write_str("fork,").unwrap();
            }
        }
        if let Some(s) = self.silent {
            if s {
                f.write_str("silent").unwrap();
            }
        }
        f.write_str("]")
    }
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawMimetype {
    entry: Option<Vec<JoshutoMimetypeEntry>>,
    extension: Option<HashMap<String, Vec<usize>>>,
    mimetype: Option<HashMap<String, Vec<usize>>>,
}

impl Flattenable<JoshutoMimetype> for JoshutoRawMimetype {
    fn flatten(self) -> JoshutoMimetype {
        let entry_all = self.entry.unwrap_or_default();
        let mut entries = HashMap::with_capacity(entry_all.len());
        for entry in entry_all {
            entries.insert(entry.id, entry);
        }
        let extension = self.extension.unwrap_or_default();
        let mimetype = self.mimetype.unwrap_or_default();

        JoshutoMimetype {
            entries,
            extension,
            mimetype,
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
    pub fn get_config() -> JoshutoMimetype {
        parse_config_file::<JoshutoRawMimetype, JoshutoMimetype>(MIMETYPE_FILE)
            .unwrap_or_else(JoshutoMimetype::default)
    }

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

impl std::default::Default for JoshutoMimetype {
    fn default() -> Self {
        JoshutoMimetype {
            entries: HashMap::new(),
            mimetype: HashMap::new(),
            extension: HashMap::new(),
        }
    }
}
