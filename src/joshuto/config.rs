extern crate whoami;
extern crate toml;
extern crate xdg;

use std::fs;
use std::process;

use joshuto;
use joshuto::sort;

#[derive(Debug, Deserialize)]
pub struct JoshutoRawConfig {
    show_hidden: Option<bool>,
    sort_type: Option<String>,
    column_ratio: Option<[usize; 3]>,
}

impl JoshutoRawConfig {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawConfig {
            show_hidden: Some(false),
            sort_type: Some(String::from("natural")),
            column_ratio: Some([1, 3, 4]),
        }
    }

    pub fn flatten(self) -> JoshutoConfig
    {
        let username : String = whoami::username();
        let hostname : String = whoami::hostname();

        let column_ratio = match self.column_ratio {
            Some(s) => (s[0], s[1], s[2]),
            None => (1, 3, 4),
            };

        let show_hidden: bool = match self.show_hidden {
            Some(s) => s,
            None => false,
            };

        let sort_struct = sort::SortStruct {
                show_hidden,
                folders_first: true,
                case_sensitive: false,
                reverse: false,
            };

        let sort_type: sort::SortType = match self.sort_type {
            Some(s) => {
                match s.as_str() {
                    "natural" => sort::SortType::SortNatural(sort_struct),
                    "mtime" => sort::SortType::SortMtime(sort_struct),
                    _ => sort::SortType::SortNatural(sort_struct),
                }
            }
            _ => sort::SortType::SortNatural(sort_struct),
            };

        JoshutoConfig {
            username,
            hostname,
            sort_type,
            column_ratio,
        }
    }
}

#[derive(Debug)]
pub struct JoshutoConfig {
    pub username: String,
    pub hostname: String,
    pub sort_type: joshuto::sort::SortType,
    pub column_ratio: (usize, usize, usize),
}

impl JoshutoConfig {

    pub fn new() -> Self
    {
        let sort_struct = sort::SortStruct {
                show_hidden: false,
                folders_first: true,
                case_sensitive: false,
                reverse: false,
            };
        let sort_type = sort::SortType::SortNatural(sort_struct);

        let username : String = whoami::username();
        let hostname : String = whoami::hostname();

        JoshutoConfig {
            username,
            hostname,
            sort_type,
            column_ratio: (1, 3, 4),
        }
    }

    fn read_config() -> Option<JoshutoRawConfig>
    {
        let dirs = xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "").unwrap();

        let config_path = dirs.find_config_file(::CONFIG_FILE)?;
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
        match JoshutoConfig::read_config() {
            Some(config) => {
                config.flatten()
            }
            None => {
                JoshutoConfig::new()
            }
        }
    }
}
