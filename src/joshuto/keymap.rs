extern crate fs_extra;
extern crate xdg;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process;

use joshuto::command;
use joshuto::command::*;
use joshuto::keymapll::Keycode;

const MAP_COMMAND: &str = "map";
// const ALIAS_COMMAND: &str = "alias";
// const NEWTAB_COMMAND: &str = "newtab";

const COMMENT_DELIMITER: char = '#';

/* #define KEY_ALT(x) KEY_F(60) + (x - 'A') */

#[derive(Debug)]
pub struct JoshutoKeymap {
    pub keymaps: HashMap<i32, CommandKeybind>,
}

impl JoshutoKeymap {
    pub fn new() -> Self
    {
        let mut keymaps: HashMap<i32, CommandKeybind> = HashMap::new();

        // quit
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::Quit::new()));
        keymaps.insert(Keycode::LOWER_Q as i32, command);

        // up
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMove::new(-1)));
        keymaps.insert(Keycode::UP as i32, command);
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMove::new(-1)));
        keymaps.insert(Keycode::LOWER_K as i32, command);

        // down
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMove::new(1)));
        keymaps.insert(Keycode::DOWN as i32, command);
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMove::new(1)));
        keymaps.insert(Keycode::LOWER_J as i32, command);

        // left
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::ParentDirectory::new()));
        keymaps.insert(Keycode::LEFT as i32, command);
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::ParentDirectory::new()));
        keymaps.insert(Keycode::LOWER_H as i32, command);

        // right
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::OpenFile::new()));
        keymaps.insert(Keycode::RIGHT as i32, command);
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::OpenFile::new()));
        keymaps.insert(Keycode::LOWER_L as i32, command);
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::OpenFile::new()));
        keymaps.insert(Keycode::ENTER as i32, command);

        let command = CommandKeybind::SimpleKeybind(
            Box::new(command::OpenFileWith::new()));
        keymaps.insert(Keycode::LOWER_R as i32, command);

        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMovePageUp::new()));
        keymaps.insert(Keycode::PPAGE as i32, command);

        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMovePageDown::new()));
        keymaps.insert(Keycode::NPAGE as i32, command);

        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMoveHome::new()));
        keymaps.insert(Keycode::HOME as i32, command);

        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMoveEnd::new()));
        keymaps.insert(Keycode::END as i32, command);

        let command = CommandKeybind::SimpleKeybind(
            Box::new(command::DeleteFiles::new()));
        keymaps.insert(Keycode::DELETE as i32, command);

        let command = CommandKeybind::SimpleKeybind(
            Box::new(command::RenameFile::new(command::RenameFileMethod::Append)));
        keymaps.insert(Keycode::LOWER_A as i32, command);

        {
            let mut subkeymap: HashMap<i32, CommandKeybind> = HashMap::new();
            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::ToggleHiddenFiles::new()));
            subkeymap.insert(Keycode::LOWER_H as i32, command);

            let command = CommandKeybind::CompositeKeybind(subkeymap);
            keymaps.insert(Keycode::LOWER_Z as i32, command);
        }

        {
            let mut subkeymap: HashMap<i32, CommandKeybind> = HashMap::new();
            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CutFiles::new()));
            subkeymap.insert(Keycode::LOWER_D as i32, command);

            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::DeleteFiles::new()));
            subkeymap.insert(Keycode::UPPER_D as i32, command);

            let command = CommandKeybind::CompositeKeybind(subkeymap);
            keymaps.insert(Keycode::LOWER_D as i32, command);
        }

        {
            let mut subkeymap: HashMap<i32, CommandKeybind> = HashMap::new();

            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CopyFiles::new()));
            subkeymap.insert(Keycode::LOWER_Y as i32, command);

            let command = CommandKeybind::CompositeKeybind(subkeymap);
            keymaps.insert(Keycode::LOWER_Y as i32, command);
        }

        {
            let mut subkeymap: HashMap<i32, CommandKeybind> = HashMap::new();

            let options = fs_extra::dir::CopyOptions::new();
            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::PasteFiles::new(options)));
            subkeymap.insert(Keycode::LOWER_P as i32, command);

            let mut options = fs_extra::dir::CopyOptions::new();
            options.overwrite = true;
            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::PasteFiles::new(options)));
            subkeymap.insert(Keycode::UPPER_P as i32, command);

            let command = CommandKeybind::CompositeKeybind(subkeymap);
            keymaps.insert(Keycode::LOWER_P as i32, command);
        }

        JoshutoKeymap {
            keymaps,
        }
    }

    fn insert_keycommand(map: &mut HashMap<i32, CommandKeybind>,
            keycommand: Box<dyn JoshutoCommand>, keys: &[&str])
    {
        if keys.len() == 1 {
            match Keycode::from_str(keys[0]) {
                Some(s) => {
                    map.insert(s as i32, CommandKeybind::SimpleKeybind(keycommand));
                },
                None => {}
            }
        } else {
            match Keycode::from_str(keys[0]) {
                Some(s) => {
                    let mut new_map: HashMap<i32, CommandKeybind>;
                    match map.remove(&(s.clone() as i32)) {
                        Some(CommandKeybind::CompositeKeybind(mut m)) => {
                            new_map = m;
                        },
                        Some(_) => {
                            eprintln!("Error: Keybindings ambiguous");
                            process::exit(1);
                        },
                        None => {
                            new_map = HashMap::new();
                        }
                    }
                    JoshutoKeymap::insert_keycommand(&mut new_map, keycommand, &keys[1..]);
                    let composite_command = CommandKeybind::CompositeKeybind(new_map);
                    map.insert(s as i32, composite_command);
                },
                None => {}
            }
        }
    }

    fn parse_line(map: &mut HashMap<i32, CommandKeybind>, line: String)
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
        line.push('\n');

        let args: Vec<&str> = command::split_shell_style(&line);

        if args.len() == 0 {
            return;
        }

        match args[0] {
            MAP_COMMAND => {
                let keys_vec: Vec<&str> = args[1].split(',').collect();
                match command::from_args(&args[2..]) {
                    Some(command) => {
                        JoshutoKeymap::insert_keycommand(map, command, &keys_vec[..]);
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
        match xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "") {
            Ok(dirs) => {
                let config_path = dirs.find_config_file(::KEYMAP_FILE)?;
                match fs::File::open(config_path) {
                    Ok(f) => {
                        let mut keymaps: HashMap<i32, CommandKeybind> = HashMap::new();
                        let mut reader = io::BufReader::new(f);
                        for line in reader.lines() {
                            if let Ok(mut line) = line {
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

