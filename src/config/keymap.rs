use serde_derive::Deserialize;
use std::collections::{hash_map::Entry, HashMap};
use std::process::exit;

use crate::commands::{self, CommandKeybind, JoshutoCommand};
use super::{parse_config_file, ConfigStructure, Flattenable};
use crate::KEYMAP_FILE;

pub const BACKSPACE: i32 = 0x7F;
pub const TAB: i32 = 0x9;
pub const ENTER: i32 = 0xA;
pub const ESCAPE: i32 = 0x1B;

/* #define KEY_ALT(x) KEY_F(60) + (x - 'A') */

#[derive(Debug, Deserialize)]
struct JoshutoMapCommand {
    pub keys: Vec<i32>,
    pub command: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct JoshutoRawKeymap {
    mapcommand: Option<Vec<JoshutoMapCommand>>,
}

impl Flattenable<JoshutoKeymap> for JoshutoRawKeymap {
    fn flatten(self) -> JoshutoKeymap {
        match self.mapcommand {
            None => JoshutoKeymap::new(),
            Some(maps) => {
                let mut keymaps = JoshutoKeymap::new();
                maps.iter().for_each(|m| {
                    match commands::from_args(m.command.as_str(), m.args.as_ref()) {
                        Ok(command) => insert_keycommand(&mut keymaps, command, &m.keys[..]),
                        Err(e) => eprintln!("{}", e),
                    }
                });
                keymaps
            }
        }
    }
}

pub type JoshutoKeymap = HashMap<i32, CommandKeybind>;

impl ConfigStructure for JoshutoKeymap {
    fn get_config() -> Self {
        parse_config_file::<JoshutoRawKeymap, JoshutoKeymap>(KEYMAP_FILE)
            .unwrap_or_else(JoshutoKeymap::default)
    }
}

fn insert_keycommand(
    map: &mut JoshutoKeymap,
    keycommand: Box<JoshutoCommand>,
    keys: &[i32],
) {
    match keys.len() {
        0 => {}
        1 => match map.entry(keys[0]) {
            Entry::Occupied(_) => {
                eprintln!("Error: Keybindings ambiguous");
                exit(1);
            }
            Entry::Vacant(entry) => {
                entry.insert(CommandKeybind::SimpleKeybind(keycommand));
            }
        },
        _ => match map.entry(keys[0]) {
            Entry::Occupied(mut entry) => match entry.get_mut() {
                CommandKeybind::CompositeKeybind(ref mut m) => {
                    insert_keycommand(m, keycommand, &keys[1..])
                }
                _ => {
                    eprintln!("Error: Keybindings ambiguous");
                    exit(1);
                }
            },
            Entry::Vacant(entry) => {
                let mut new_map = HashMap::new();
                insert_keycommand(&mut new_map, keycommand, &keys[1..]);
                let composite_command = CommandKeybind::CompositeKeybind(new_map);
                entry.insert(composite_command);
            }
        },
    }
}
