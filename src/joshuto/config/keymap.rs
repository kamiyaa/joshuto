extern crate fs_extra;
extern crate toml;
extern crate xdg;

use std::collections::HashMap;
use std::process;

use joshuto::command;
use joshuto::config::{parse_config_file, Flattenable};
use KEYMAP_FILE;

pub const BACKSPACE: i32 = 0x7F;
pub const TAB: i32 = 0x9;
pub const ENTER: i32 = 0xA;
pub const ESCAPE: i32 = 0x1B;

/* #define KEY_ALT(x) KEY_F(60) + (x - 'A') */

#[derive(Debug, Deserialize)]
struct JoshutoMapCommand {
    pub keys: Vec<String>,
    pub command: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct JoshutoRawKeymap {
    mapcommand: Option<Vec<JoshutoMapCommand>>,
}

impl Flattenable<JoshutoKeymap> for JoshutoRawKeymap {
    fn flatten(self) -> JoshutoKeymap {
        let mut keymaps: HashMap<i32, command::CommandKeybind> = HashMap::new();
        if let Some(maps) = self.mapcommand {
            for mapcommand in maps {
                match command::from_args(mapcommand.command.as_str(), mapcommand.args.as_ref()) {
                    Some(command) => {
                        insert_keycommand(&mut keymaps, command, &mapcommand.keys[..]);
                    }
                    None => {
                        println!("Unknown command: {}", mapcommand.command);
                    }
                }
            }
        }
        JoshutoKeymap { keymaps }
    }
}

#[derive(Debug)]
pub struct JoshutoKeymap {
    pub keymaps: HashMap<i32, command::CommandKeybind>,
}

impl JoshutoKeymap {
    pub fn new() -> Self {
        let keymaps = HashMap::new();
        JoshutoKeymap { keymaps }
    }

    pub fn get_config() -> JoshutoKeymap {
        parse_config_file::<JoshutoRawKeymap, JoshutoKeymap>(KEYMAP_FILE)
            .unwrap_or_else(|| JoshutoKeymap::new())
    }
}

fn insert_keycommand(
    map: &mut HashMap<i32, command::CommandKeybind>,
    keycommand: Box<dyn command::JoshutoCommand>,
    keys: &[String],
) {
    if keys.len() == 1 {
        match key_to_i32(&keys[0]) {
            Some(s) => {
                map.insert(s, command::CommandKeybind::SimpleKeybind(keycommand));
            }
            None => {}
        }
    } else {
        match key_to_i32(&keys[0]) {
            Some(s) => {
                let mut new_map: HashMap<i32, command::CommandKeybind>;
                match map.remove(&s) {
                    Some(command::CommandKeybind::CompositeKeybind(mut m)) => {
                        new_map = m;
                    }
                    Some(_) => {
                        eprintln!("Error: Keybindings ambiguous");
                        process::exit(1);
                    }
                    None => {
                        new_map = HashMap::new();
                    }
                }
                insert_keycommand(&mut new_map, keycommand, &keys[1..]);
                let composite_command = command::CommandKeybind::CompositeKeybind(new_map);
                map.insert(s as i32, composite_command);
            }
            None => {}
        }
    }
}

pub fn key_to_i32(keycode: &str) -> Option<i32> {
    if keycode.len() == 1 {
        for ch in keycode.chars() {
            if ch.is_ascii() {
                return Some(ch as i32);
            }
        }
        return None;
    } else {
        match keycode {
            "Tab" => Some(TAB),
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
