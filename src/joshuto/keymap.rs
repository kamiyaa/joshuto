use std::str;
use std::collections::HashMap;
use std::process;
use std::slice;

use joshuto::keymapll::JoshutoCommand;
use joshuto::keymapll::Keycode;

#[derive(Debug, Deserialize)]
pub struct JoshutoRawKeymaps {
    keymaps: Option<HashMap<String, Vec<Vec<String>>>>,
}

impl JoshutoRawKeymaps {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawKeymaps {
            keymaps: None,
        }
    }

    pub fn flatten(self) -> JoshutoKeymaps
    {
        let keymaps = match self.keymaps {
                Some(s) => {
                    s
                }
                None => {
                    HashMap::new()
                },
            };

        JoshutoKeymaps {
            keymaps: JoshutoRawKeymaps::unflatten_hashmap(keymaps)
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
                            JoshutoRawKeymaps::insert_keycommand(&mut new_map, &mut keys,
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
                print!("{:?}+", key);
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

                JoshutoRawKeymaps::insert_keycommand(&mut new_map, keys, new_key, keycommand);
                map.insert(key as i32, JoshutoCommand::CompositeKeybind(new_map));
            }
            None => {
                println!("{:?} -> {:?}", key, keycommand);
                map.insert(key as i32, keycommand);
            }
        }
    }
}

#[derive(Debug)]
pub struct JoshutoKeymaps {
    pub keymaps: HashMap<i32, JoshutoCommand>,
}

impl JoshutoKeymaps {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoKeymaps {
            keymaps: HashMap::new(),
        }
    }
}
