extern crate fs_extra;
extern crate xdg;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process;

use joshuto::command;
use joshuto::command::*;

const MAP_COMMAND: &str = "map";
// const ALIAS_COMMAND: &str = "alias";
// const NEWTAB_COMMAND: &str = "newtab";

const COMMENT_DELIMITER: char = '#';

pub const BACKSPACE: i32 = 0x7F;
pub const TAB: i32 = 0x9;
pub const ENTER: i32 = 0xA;
pub const ESCAPE: i32 = 0x1B;

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
        keymaps.insert('q' as i32, command);

        // up
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMove::new(-1)));
        keymaps.insert(ncurses::KEY_UP, command);
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMove::new(-1)));
        keymaps.insert('k' as i32, command);

        // down
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMove::new(1)));
        keymaps.insert(ncurses::KEY_DOWN, command);
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMove::new(1)));
        keymaps.insert('j' as i32, command);

        // left
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::ParentDirectory::new()));
        keymaps.insert(ncurses::KEY_LEFT, command);
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::ParentDirectory::new()));
        keymaps.insert('h' as i32, command);

        // right
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::OpenFile::new()));
        keymaps.insert(ncurses::KEY_RIGHT, command);
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::OpenFile::new()));
        keymaps.insert('l' as i32, command);
        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::OpenFile::new()));
        keymaps.insert(ENTER, command);

        let command = CommandKeybind::SimpleKeybind(
            Box::new(command::OpenFileWith::new()));
        keymaps.insert('r' as i32, command);

        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMovePageUp::new()));
        keymaps.insert(ncurses::KEY_PPAGE, command);

        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMovePageDown::new()));
        keymaps.insert(ncurses::KEY_NPAGE, command);

        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMoveHome::new()));
        keymaps.insert(ncurses::KEY_HOME, command);

        let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CursorMoveEnd::new()));
        keymaps.insert(ncurses::KEY_END, command);

        let command = CommandKeybind::SimpleKeybind(
            Box::new(command::DeleteFiles::new()));
        keymaps.insert(ncurses::KEY_DC, command);

        let command = CommandKeybind::SimpleKeybind(
            Box::new(command::RenameFile::new(command::RenameFileMethod::Append)));
        keymaps.insert('a' as i32, command);

        {
            let mut subkeymap: HashMap<i32, CommandKeybind> = HashMap::new();
            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::ToggleHiddenFiles::new()));
            subkeymap.insert('h' as i32, command);

            let command = CommandKeybind::CompositeKeybind(subkeymap);
            keymaps.insert('z' as i32, command);
        }

        {
            let mut subkeymap: HashMap<i32, CommandKeybind> = HashMap::new();
            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CutFiles::new()));
            subkeymap.insert('d' as i32, command);

            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::DeleteFiles::new()));
            subkeymap.insert('D' as i32, command);

            let command = CommandKeybind::CompositeKeybind(subkeymap);
            keymaps.insert('d' as i32, command);
        }

        {
            let mut subkeymap: HashMap<i32, CommandKeybind> = HashMap::new();

            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::CopyFiles::new()));
            subkeymap.insert('y' as i32, command);

            let command = CommandKeybind::CompositeKeybind(subkeymap);
            keymaps.insert('y' as i32, command);
        }

        {
            let mut subkeymap: HashMap<i32, CommandKeybind> = HashMap::new();

            let options = fs_extra::dir::CopyOptions::new();
            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::PasteFiles::new(options)));
            subkeymap.insert('p' as i32, command);

            let mut options = fs_extra::dir::CopyOptions::new();
            options.overwrite = true;
            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::PasteFiles::new(options)));
            subkeymap.insert('o' as i32, command);

            let command = CommandKeybind::CompositeKeybind(subkeymap);
            keymaps.insert('p' as i32, command);
        }

        {
            let mut subkeymap: HashMap<i32, CommandKeybind> = HashMap::new();
            let command = CommandKeybind::SimpleKeybind(
                Box::new(command::NewDirectory::new()));
            subkeymap.insert('k' as i32, command);

            let command = CommandKeybind::CompositeKeybind(subkeymap);
            keymaps.insert('m' as i32, command);
        }

        JoshutoKeymap {
            keymaps,
        }
    }

    pub fn from_str(keycode: &str) -> Option<i32>
    {
        if keycode.len() == 1 {
            for ch in keycode.chars() {
                if ch.is_ascii() {
                    return Some(ch as i32);
                }
            }
            return None;
        } else {
            match keycode {
                "Comma" => Some(',' as i32),
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

                "Insert" => Some(ncurses::KEY_IC),           /* insert-character key */
                "PageUp" => Some(ncurses::KEY_PPAGE),        /* next-page key */
                "PageDown" => Some(ncurses::KEY_NPAGE),      /* previous-page key */
                "PrintScreen" => Some(ncurses::KEY_PRINT),    /* print key */

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

    fn insert_keycommand(map: &mut HashMap<i32, CommandKeybind>,
            keycommand: Box<dyn JoshutoCommand>, keys: &[&str])
    {
        if keys.len() == 1 {
            match Self::from_str(keys[0]) {
                Some(s) => {
                    map.insert(s, CommandKeybind::SimpleKeybind(keycommand));
                },
                None => {}
            }
        } else {
            match Self::from_str(keys[0]) {
                Some(s) => {
                    let mut new_map: HashMap<i32, CommandKeybind>;
                    match map.remove(&s) {
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
