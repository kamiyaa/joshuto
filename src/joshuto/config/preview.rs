extern crate toml;
extern crate xdg;

use std::fmt;
use std::fs;
use std::collections::HashMap;
use std::process;

#[derive(Debug, Deserialize)]
pub struct JoshutoPreviewEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawPreview {
    pub mimetype: Option<HashMap<String, JoshutoPreviewEntry>>,
    pub extension: Option<HashMap<String, JoshutoPreviewEntry>>,
}

impl JoshutoRawPreview {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawPreview {
            mimetype: None,
            extension: None,
        }
    }

    pub fn flatten(self) -> JoshutoPreview
    {
        let mimetype = self.mimetype.unwrap_or(HashMap::new());
        let extension = self.extension.unwrap_or(HashMap::new());

        JoshutoPreview {
            mimetype,
            extension,
        }
    }
}

#[derive(Debug)]
pub struct JoshutoPreview {
    pub mimetype: HashMap<String, JoshutoPreviewEntry>,
    pub extension: HashMap<String, JoshutoPreviewEntry>,
}

impl JoshutoPreview {

    pub fn new() -> Self
    {
        JoshutoPreview {
            mimetype: HashMap::new(),
            extension: HashMap::new(),
        }
    }

    fn read_config() -> Option<JoshutoRawPreview>
    {
        match xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "") {
            Ok(dirs) => {
                let config_path = dirs.find_config_file(::PREVIEW_FILE)?;
                match fs::read_to_string(&config_path) {
                    Ok(config_contents) => {
                        match toml::from_str(&config_contents) {
                            Ok(config) => {
                                Some(config)
                            },
                            Err(e) => {
                                eprintln!("Error parsing preview file: {}", e);
                                process::exit(1);
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    },
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                None
            },
        }
    }

    pub fn get_config() -> Self
    {
        match Self::read_config() {
            Some(config) => {
                config.flatten()
            }
            None => {
                Self::new()
            }
        }
    }
}
