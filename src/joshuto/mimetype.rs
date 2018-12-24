extern crate toml;
extern crate xdg;

use std::fs;
use std::collections::HashMap;
use std::process;

#[derive(Debug, Deserialize)]
pub struct JoshutoRawMimetype {
    mimetypes: Option<HashMap<String, Vec<Vec<String>>>>,
}

impl JoshutoRawMimetype {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawMimetype {
            mimetypes: None,
        }
    }

    pub fn flatten(self) -> JoshutoMimetype
    {
        let mimetypes = match self.mimetypes {
            Some(s) => s,
            None => HashMap::new(),
            };

        JoshutoMimetype {
            mimetypes,
        }
    }
}

#[derive(Debug)]
pub struct JoshutoMimetype {
    pub mimetypes: HashMap<String, Vec<Vec<String>>>,
}

impl JoshutoMimetype {

    pub fn new() -> Self
    {
        JoshutoMimetype {
            mimetypes: HashMap::new(),
        }
    }

    fn read_config() -> Option<JoshutoRawMimetype>
    {
        let dirs = xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "").unwrap();

        let config_path = dirs.find_config_file(::MIMETYPE_FILE)?;
        println!("config_path: {:?}", config_path);
        match fs::read_to_string(&config_path) {
            Ok(config_contents) => {
                match toml::from_str(&config_contents) {
                    Ok(config) => {
                        Some(config)
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
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
        match JoshutoMimetype::read_config() {
            Some(config) => {
                config.flatten()
            }
            None => {
                JoshutoMimetype::new()
            }
        }
    }
}
