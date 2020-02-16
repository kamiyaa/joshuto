use std::collections::{hash_map::Entry, HashMap};

use serde_derive::Deserialize;

use termion::event::Key;

use super::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::commands::{self, CommandKeybind, JoshutoCommand};
use crate::util::key_mapping::str_to_key;
use crate::KEYMAP_FILE;

pub type JoshutoCommandMapping = HashMap<Key, CommandKeybind>;

#[derive(Debug, Deserialize)]
struct JoshutoMapCommand {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct JoshutoRawCommandMapping {
    #[serde(default)]
    mapcommand: Vec<JoshutoMapCommand>,
}

impl Flattenable<JoshutoCommandMapping> for JoshutoRawCommandMapping {
    fn flatten(self) -> JoshutoCommandMapping {
        let mut keymaps = JoshutoCommandMapping::new();
        for m in self.mapcommand {
            match commands::from_args(m.command, m.args) {
                Ok(command) => {
                    let keycodes: Vec<&str> = m.keys.iter().map(|s| s.as_str()).collect();

                    let result = insert_keycommand(&mut keymaps, command, &keycodes);
                    match result {
                        Ok(_) => {}
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Err(e) => eprintln!("{}", e.cause()),
            }
        }
        keymaps
    }
}

impl ConfigStructure for JoshutoCommandMapping {
    fn get_config() -> Self {
        parse_to_config_file::<JoshutoRawCommandMapping, JoshutoCommandMapping>(KEYMAP_FILE)
            .unwrap_or_else(Self::default)
    }
}

fn insert_keycommand(
    keymap: &mut JoshutoCommandMapping,
    keycommand: Box<dyn JoshutoCommand>,
    keycodes: &[&str],
) -> Result<(), String> {
    let keycode_len = keycodes.len();

    if keycode_len == 0 {
        return Ok(());
    }

    let key = match str_to_key(keycodes[0]) {
        Some(k) => k,
        None => return Err(format!("Unknown keycode: {}", keycodes[0])),
    };

    if keycode_len == 1 {
        match keymap.entry(key) {
            Entry::Occupied(_) => {
                return Err(format!("Error: Keybindings ambiguous for {}", keycommand))
            }
            Entry::Vacant(entry) => entry.insert(CommandKeybind::SimpleKeybind(keycommand)),
        };
        return Ok(());
    }

    match keymap.entry(key) {
        Entry::Occupied(mut entry) => match entry.get_mut() {
            CommandKeybind::CompositeKeybind(ref mut m) => {
                return insert_keycommand(m, keycommand, &keycodes[1..])
            }
            _ => return Err(format!("Error: Keybindings ambiguous for {}", keycommand)),
        },
        Entry::Vacant(entry) => {
            let mut new_map = JoshutoCommandMapping::new();
            let result = insert_keycommand(&mut new_map, keycommand, &keycodes[1..]);
            match result {
                Ok(_) => {
                    let composite_command = CommandKeybind::CompositeKeybind(new_map);
                    entry.insert(composite_command);
                }
                _ => {}
            }
            return result;
        }
    }
}
