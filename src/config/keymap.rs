use serde_derive::Deserialize;
use std::collections::{hash_map::Entry, HashMap};
use std::process::exit;

use super::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::commands::{self, CommandKeybind, JoshutoCommand};
use crate::KEYMAP_FILE;

pub const ESCAPE: i32 = 0x1B;

/* #define KEY_ALT(x) KEY_F(60) + (x - 'A') */

const fn default_up() -> i32 {
    ncurses::KEY_UP
}

const fn default_down() -> i32 {
    ncurses::KEY_DOWN
}

const fn default_left() -> i32 {
    ncurses::KEY_LEFT
}

const fn default_right() -> i32 {
    ncurses::KEY_RIGHT
}

const fn default_home() -> i32 {
    ncurses::KEY_HOME
}

const fn default_end() -> i32 {
    ncurses::KEY_END
}

const fn default_backspace() -> i32 {
    ncurses::KEY_BACKSPACE
}

const fn default_delete() -> i32 {
    ncurses::KEY_DC
}

const fn default_enter() -> i32 {
    '\n' as i32
}

const fn default_escape() -> i32 {
    ESCAPE
}

const fn default_tab() -> i32 {
    '\t' as i32
}

#[derive(Debug, Deserialize)]
struct JoshutoMapCommand {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub keys: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct JoshutoRawKeymapping {
    #[serde(default)]
    keymaps: JoshutoKeyMapping,
    #[serde(skip)]
    mapcommand: Vec<JoshutoMapCommand>,
}

#[derive(Debug, Deserialize)]
pub struct JoshutoKeyMapping {
    #[serde(default = "default_up")]
    pub up: i32,
    #[serde(default = "default_down")]
    pub down: i32,
    #[serde(default = "default_left")]
    pub left: i32,
    #[serde(default = "default_right")]
    pub right: i32,
    #[serde(default = "default_home")]
    pub home: i32,
    #[serde(default = "default_end")]
    pub end: i32,
    /*
        #[serde(default = "default_up")]
        pub page_up: i32,
        #[serde(default = "default_up")]
        pub page_down: i32,
    */
    #[serde(default = "default_backspace")]
    pub backspace: i32,
    #[serde(default = "default_delete")]
    pub delete: i32,
    #[serde(default = "default_enter")]
    pub enter: i32,
    #[serde(default = "default_escape")]
    pub escape: i32,
    #[serde(default = "default_tab")]
    pub tab: i32,
}

impl std::default::Default for JoshutoKeyMapping {
    fn default() -> Self {
        JoshutoKeyMapping {
            up: default_up(),
            down: default_down(),
            left: default_left(),
            right: default_right(),
            home: default_home(),
            end: default_end(),
            backspace: default_backspace(),
            delete: default_delete(),
            enter: default_enter(),
            escape: default_escape(),
            tab: default_tab(),
        }
    }
}

impl Flattenable<JoshutoKeyMapping> for JoshutoRawKeymapping {
    fn flatten(self) -> JoshutoKeyMapping {
        self.keymaps
    }
}

impl ConfigStructure for JoshutoKeyMapping {
    fn get_config() -> Self {
        parse_to_config_file::<JoshutoRawKeymapping, JoshutoKeyMapping>(KEYMAP_FILE)
            .unwrap_or_else(Self::default)
    }
}

#[derive(Debug, Deserialize)]
struct JoshutoRawCommandMapping {
    #[serde(skip)]
    keymaps: JoshutoKeyMapping,
    #[serde(default)]
    mapcommand: Vec<JoshutoMapCommand>,
}

impl Flattenable<JoshutoCommandMapping> for JoshutoRawCommandMapping {
    fn flatten(self) -> JoshutoCommandMapping {
        let mut keymaps = JoshutoCommandMapping::new();
        for m in self.mapcommand {
            match commands::from_args(m.command, m.args) {
                Ok(command) => insert_keycommand(&mut keymaps, command, &m.keys[..]),
                Err(e) => eprintln!("{}", e.cause()),
            }
        }
        keymaps
    }
}

pub type JoshutoCommandMapping = HashMap<i32, CommandKeybind>;

impl ConfigStructure for JoshutoCommandMapping {
    fn get_config() -> Self {
        parse_to_config_file::<JoshutoRawCommandMapping, JoshutoCommandMapping>(KEYMAP_FILE)
            .unwrap_or_else(Self::default)
    }
}

fn insert_keycommand(
    keymap: &mut JoshutoCommandMapping,
    keycommand: Box<JoshutoCommand>,
    keycodes: &[i32],
) {
    match keycodes.len() {
        0 => {}
        1 => match keymap.entry(keycodes[0]) {
            Entry::Occupied(_) => {
                eprintln!("Error: Keybindings ambiguous for {}", keycommand);
                exit(1);
            }
            Entry::Vacant(entry) => {
                entry.insert(CommandKeybind::SimpleKeybind(keycommand));
            }
        },
        _ => match keymap.entry(keycodes[0]) {
            Entry::Occupied(mut entry) => match entry.get_mut() {
                CommandKeybind::CompositeKeybind(ref mut m) => {
                    insert_keycommand(m, keycommand, &keycodes[1..])
                }
                _ => {
                    eprintln!("Error: Keybindings ambiguous for {}", keycommand);
                    exit(1);
                }
            },
            Entry::Vacant(entry) => {
                let mut new_map = JoshutoCommandMapping::new();
                insert_keycommand(&mut new_map, keycommand, &keycodes[1..]);
                let composite_command = CommandKeybind::CompositeKeybind(new_map);
                entry.insert(composite_command);
            }
        },
    }
}
