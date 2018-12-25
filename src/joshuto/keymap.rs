extern crate xdg;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process;
use std::slice;
use std::ops;

use joshuto::keymapll::JoshutoCommand;
use joshuto::keymapll::Keycode;

const MAP_COMMAND: &str = "map";
const ALIAS_COMMAND: &str = "alias";

macro_rules! new_keymap {

    ($($key: expr => $val: expr),*) => [
        {
            let mut map: HashMap<i32, JoshutoCommand> = HashMap::new();

            $(
                map.insert($key as i32, $val);
            )*

            map
        }
    ]
}

#[derive(Debug)]
pub struct JoshutoKeymap {
    pub keymaps: HashMap<i32, JoshutoCommand>,
}

impl JoshutoKeymap {
    pub fn new() -> Self
    {
        let keymaps: HashMap<i32, JoshutoCommand> =
            new_keymap![
                Keycode::UP => JoshutoCommand::CursorMove(-1),
                Keycode::DOWN => JoshutoCommand::CursorMove(-1),
                Keycode::LEFT => JoshutoCommand::ParentDirectory,
                Keycode::RIGHT => JoshutoCommand::Open,
                Keycode::HOME => JoshutoCommand::CursorMoveHome,
                Keycode::END => JoshutoCommand::CursorMoveEnd,
                'a' => JoshutoCommand::RenameFile,
                'd' => JoshutoCommand::CompositeKeybind(
                    new_keymap![
                    'd' => JoshutoCommand::CutFiles,
                    'D' => JoshutoCommand::DeleteFiles]
                ),
                'k' => JoshutoCommand::CursorMove(-1),
                'j' => JoshutoCommand::CursorMove(1),
                'h' => JoshutoCommand::ParentDirectory,
                'l' => JoshutoCommand::Open,
                'q' => JoshutoCommand::Quit,
                'y' => JoshutoCommand::CompositeKeybind(
                    new_keymap![
                    'y' => JoshutoCommand::CopyFiles]
                ),
                'p' => JoshutoCommand::CompositeKeybind(
                    new_keymap![
                    'p' => JoshutoCommand::PasteFiles]
                ),
                'z' => JoshutoCommand::CompositeKeybind(
                    new_keymap![
                    'h' => JoshutoCommand::ToggleHiddenFiles]
                )
            ];

        JoshutoKeymap {
            keymaps,
        }
    }

    fn insert_keycommand(map: &mut HashMap<i32, JoshutoCommand>,
            keys: &[&str], keycommand: JoshutoCommand)
    {
        let keys_len = keys.len();

        if keys_len == 0 {
            return;
        }
        else if keys.len() == 1 {
            match Keycode::from_str(keys[0]) {
                Some(s) => {
                    map.insert(s as i32, keycommand);
                },
                None => {}
            }
        } else {
            match Keycode::from_str(keys[0]) {
                Some(s) => {
                    let mut new_map: HashMap<i32, JoshutoCommand>;
                    match map.remove(&(s.clone() as i32)) {
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
                    JoshutoKeymap::insert_keycommand(&mut new_map, &keys[1..], keycommand);
                    let composite_command = JoshutoCommand::CompositeKeybind(new_map);
                    map.insert(s as i32, composite_command);
                },
                None => {}
            }
        }
    }

    fn parse_line(map: &mut HashMap<i32, JoshutoCommand>, line: String)
    {
        let mut line = line;
        {
            let mut trunc_index: Option<usize> = None;
            for (index, ch) in line.char_indices() {
                if ch == '#' {
                    trunc_index = Some(index);
                    break;
                }
            }
            if let Some(trunc_index) = trunc_index {
                line.truncate(trunc_index as usize);
            }
        }
        let splitargs: Vec<&str> = line.as_str().split_whitespace().collect();
        let splitargs_len = splitargs.len();
        if splitargs_len > 0 {
            match splitargs[0] {
                MAP_COMMAND => {
                    if splitargs_len > 2 {
                        let keys: Vec<&str> = splitargs[1].split(',').collect();
                        let keycommand_args = &splitargs[2..];
                        match JoshutoCommand::from_args(&keycommand_args) {
                            Some(s) => {
                                JoshutoKeymap::insert_keycommand(map, &keys[..], s);
                            },
                            None => {
                                println!("Unknown command: {:?}", keycommand_args);
                            },
                        }
                    }
                },
                _ => {},
            }
        }
    }

    fn read_config() -> Option<JoshutoKeymap>
    {
        let mut keymaps: HashMap<i32, JoshutoCommand> = HashMap::new();

        let dirs = xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "").unwrap();

        let config_path = dirs.find_config_file(::KEYMAP_FILE)?;
        if let Ok(f) = fs::File::open(config_path) {
            let mut reader = io::BufReader::new(f);
            for line in reader.lines() {
                if let Ok(mut line) = line {
                    JoshutoKeymap::parse_line(&mut keymaps, line);
                }
            }
            Some(JoshutoKeymap {
                keymaps,
            })
        } else {
            None
        }
    }

    pub fn get_config() -> JoshutoKeymap
    {
        match JoshutoKeymap::read_config() {
            Some(config) => {
                config
            }
            None => {
                println!("somethign happened");
                JoshutoKeymap::new()
            }
        }
    }
}
