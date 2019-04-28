use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt;

use crate::config::{parse_config_file, Flattenable};
use crate::MIMETYPE_FILE;

#[derive(Debug, Deserialize)]
pub struct JoshutoMimetypeEntry {
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
    mimetype: Option<HashMap<String, Vec<JoshutoMimetypeEntry>>>,
    extension: Option<HashMap<String, Vec<JoshutoMimetypeEntry>>>,
}

impl Flattenable<JoshutoMimetype> for JoshutoRawMimetype {
    fn flatten(self) -> JoshutoMimetype {
        let mimetype = self.mimetype.unwrap_or_default();
        let extension = self.extension.unwrap_or_default();

        JoshutoMimetype {
            mimetype,
            extension,
        }
    }
}

#[derive(Debug)]
pub struct JoshutoMimetype {
    pub mimetype: HashMap<String, Vec<JoshutoMimetypeEntry>>,
    pub extension: HashMap<String, Vec<JoshutoMimetypeEntry>>,
}

impl JoshutoMimetype {
    pub fn get_config() -> JoshutoMimetype {
        parse_config_file::<JoshutoRawMimetype, JoshutoMimetype>(MIMETYPE_FILE)
            .unwrap_or_else(JoshutoMimetype::default)
    }
}

impl std::default::Default for JoshutoMimetype {
    fn default() -> Self {
        JoshutoMimetype {
            mimetype: HashMap::new(),
            extension: HashMap::new(),
        }
    }
}
