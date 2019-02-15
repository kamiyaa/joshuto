extern crate toml;
extern crate xdg;

use std::collections::HashMap;
use std::fmt;

use joshuto::config::{parse_config_file, Flattenable};
use MIMETYPE_FILE;

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
            for arg in s {
                write!(f, " {}", arg).unwrap();
            }
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

impl JoshutoRawMimetype {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JoshutoRawMimetype {
            mimetype: None,
            extension: None,
        }
    }
}

impl Flattenable<JoshutoMimetype> for JoshutoRawMimetype {
    fn flatten(self) -> JoshutoMimetype {
        let mimetype = self.mimetype.unwrap_or(HashMap::new());
        let extension = self.extension.unwrap_or(HashMap::new());

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
    pub fn new() -> Self {
        JoshutoMimetype {
            mimetype: HashMap::new(),
            extension: HashMap::new(),
        }
    }

    pub fn get_config() -> JoshutoMimetype {
        parse_config_file::<JoshutoRawMimetype, JoshutoMimetype>(MIMETYPE_FILE)
            .unwrap_or_else(|| JoshutoMimetype::new())
    }
}
