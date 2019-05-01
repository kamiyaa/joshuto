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

pub fn key_to_i32(keycode: &str) -> Option<i32> {
    if keycode.len() == 1 {
        for ch in keycode.chars() {
            if ch.is_ascii() {
                return Some(ch as i32);
            }
        }
        None
    } else {
        match keycode {
            "Tab" => Some(TAB),
            "ShiftTab" => Some(ncurses::KEY_BTAB),
            "Space" => Some(' ' as i32),
            "Backspace" => Some(BACKSPACE),
            "Delete" => Some(ncurses::KEY_DC),
            "Enter" => Some(ENTER),
            "Escape" => Some(ESCAPE),

            "F0" => Some(ncurses::KEY_F0),
            "F1" => Some(ncurses::KEY_F1),
            "F2" => Some(ncurses::KEY_F2),
            "F3" => Some(ncurses::KEY_F3),
            "F4" => Some(ncurses::KEY_F4),
            "F5" => Some(ncurses::KEY_F5),
            "F6" => Some(ncurses::KEY_F6),
            "F7" => Some(ncurses::KEY_F7),
            "F8" => Some(ncurses::KEY_F8),
            "F9" => Some(ncurses::KEY_F9),
            "F10" => Some(ncurses::KEY_F10),
            "F11" => Some(ncurses::KEY_F11),
            "F12" => Some(ncurses::KEY_F12),
            "F13" => Some(ncurses::KEY_F13),
            "F14" => Some(ncurses::KEY_F14),
            "F15" => Some(ncurses::KEY_F15),

            "Insert" => Some(ncurses::KEY_IC), /* insert-character key */
            "PageUp" => Some(ncurses::KEY_PPAGE), /* next-page key */
            "PageDown" => Some(ncurses::KEY_NPAGE), /* previous-page key */
            "PrintScreen" => Some(ncurses::KEY_PRINT), /* print key */

            "Up" => Some(ncurses::KEY_UP),
            "Down" => Some(ncurses::KEY_DOWN),
            "Left" => Some(ncurses::KEY_LEFT),
            "Right" => Some(ncurses::KEY_RIGHT),
            "Home" => Some(ncurses::KEY_HOME),
            "End" => Some(ncurses::KEY_END),
            _ => None,
        }
    }
}
