extern crate toml;
extern crate xdg;

use std::fs;
use std::collections::HashMap;
use std::process;
use std::slice;

use joshuto::keymapll::JoshutoCommand;
use joshuto::keymapll::Keycode;

#[derive(Debug, Deserialize)]
pub struct JoshutoRawKeymap {
    keymaps: Option<HashMap<String, Vec<Vec<String>>>>,
}

impl JoshutoRawKeymap {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawKeymap {
            keymaps: None,
        }
    }

    pub fn flatten(self) -> JoshutoKeymap
    {
        let keymaps = match self.keymaps {
                Some(s) => {
                    s
                }
                None => {
                    HashMap::new()
                },
            };

        JoshutoKeymap {
            keymaps: JoshutoRawKeymap::unflatten_hashmap(keymaps)
        }
    }

    fn unflatten_hashmap(map: HashMap<String, Vec<Vec<String>>>) -> HashMap<i32, JoshutoCommand>
    {
        let mut new_map: HashMap<i32, JoshutoCommand> = HashMap::new();

        for (keycommand, keycomb) in &map {
            match JoshutoCommand::from_str(&keycommand) {
                Some(keybind) => {
                    for comb in keycomb {
                        let mut keys = comb.iter();
                        if let Some(key) = keys.next() {
                            let key = match Keycode::from_str(&key) {
                                Some(s) => s,
                                None => {
                                    eprintln!("Error: Unknown keycode for: {:?}", &keycommand);
                                    process::exit(1);
                                }
                            };
                            JoshutoRawKeymap::insert_keycommand(&mut new_map, &mut keys,
                                key, keybind.clone());
                        }
                    }
                }
                None => {
                    eprintln!("Error: Unknown command: {:?}", &keycommand);
                    process::exit(1);
                }
            }
        }
        new_map
    }

    fn insert_keycommand(map: &mut HashMap<i32, JoshutoCommand>,
            keys: &mut slice::Iter<String>, key: Keycode, keycommand: JoshutoCommand)
    {
        match keys.next() {
            Some(s) => {
                let mut new_map: HashMap<i32, JoshutoCommand>;

                let key_i32 = key.clone() as i32;
                match map.remove(&key_i32) {
                    Some(JoshutoCommand::CompositeKeybind(mut m)) => {
                        new_map = m;
                    },
                    Some(_) => {
                        eprintln!("Error: Keybindings ambiguous: {:?}", &keycommand);
                        process::exit(1);
                    },
                    None => {
                        new_map = HashMap::new();
                    }
                }
                let new_key = match Keycode::from_str(&s) {
                        Some(s) => s,
                        None => {
                            eprintln!("Error: Unknown keycode for: {:?}", &keycommand);
                            process::exit(1);
                        }
                    };

                JoshutoRawKeymap::insert_keycommand(&mut new_map, keys, new_key, keycommand);
                map.insert(key as i32, JoshutoCommand::CompositeKeybind(new_map));
            }
            None => {
                map.insert(key as i32, keycommand);
            }
        }
    }
}

#[derive(Debug)]
pub struct JoshutoKeymap {
    pub keymaps: HashMap<i32, JoshutoCommand>,
}

impl JoshutoKeymap {
    pub fn new() -> Self
    {
        JoshutoKeymap {
            keymaps: HashMap::new(),
        }
    }

    fn read_config() -> Option<JoshutoRawKeymap>
    {
        let dirs = xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "").unwrap();

        let config_path = dirs.find_config_file(::KEYMAP_FILE)?;
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

    pub fn get_config() -> JoshutoKeymap
    {
        match JoshutoKeymap::read_config() {
            Some(config) => {
                config.flatten()
            }
            None => {
                JoshutoKeymap::new()
            }
        }
    }
}
