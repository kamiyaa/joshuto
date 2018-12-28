extern crate xdg;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process;

use joshuto::command::JoshutoCommand;
use joshuto::keymapll::Keycode;

const MAP_COMMAND: &str = "map";
const ALIAS_COMMAND: &str = "alias";

const COMMENT_DELIMITER: char = '#';

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

                Keycode::SPACE => JoshutoCommand::MarkFiles{toggle: true, all: false},
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
                    'p' => JoshutoCommand::PasteFiles{overwrite: false}]
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
            keycommand: JoshutoCommand, keys: &[&str])
    {
        if keys.len() == 1 {
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
                    JoshutoKeymap::insert_keycommand(&mut new_map, keycommand, &keys[1..]);
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
            if let Some(trunc_index) = line.find(COMMENT_DELIMITER) {
                line.truncate(trunc_index as usize);
            }
        }
        if line.len() == 0 {
            return;
        }

        let args: Vec<&str> = JoshutoCommand::split_shell_style(&line);

        if args.len() == 0 {
            return;
        }
        eprintln!("args: {:?}", args);
        match args[0] {
            MAP_COMMAND => {
                let keys_vec: Vec<&str> = args[1].split(',').collect();
                match JoshutoCommand::from_args(&args[2..]) {
                    Some(s) => {
                        JoshutoKeymap::insert_keycommand(map, s, &keys_vec[..]);
                    },
                    None => {
                        println!("Unknown command: {}", args[2]);
                    },
                }
            },
            _ => eprintln!("Error: Unknown command: {}", args[0]),
        }
    }

    fn read_config() -> Option<JoshutoKeymap>
    {
        let mut keymaps: HashMap<i32, JoshutoCommand> = HashMap::new();

        match xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "") {
            Ok(dirs) => {
                let config_path = dirs.find_config_file(::KEYMAP_FILE)?;
                match fs::File::open(config_path) {
                    Ok(f) => {
                        let mut reader = io::BufReader::new(f);
                        for line in reader.lines() {
                            if let Ok(mut line) = line {
                                line.push('\n');
                                JoshutoKeymap::parse_line(&mut keymaps, line);
                            }
                        }
                        Some(JoshutoKeymap {
                            keymaps,
                        })
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
                config
            }
            None => {
                JoshutoKeymap::new()
            }
        }
    }
}
