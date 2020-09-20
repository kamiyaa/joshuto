use std::collections::{hash_map::Entry, HashMap};

use serde_derive::Deserialize;

use termion::event::Key;

use super::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::commands::{CommandKeybind, KeyCommand};
use crate::io::IOWorkerOptions;
use crate::util::key_mapping::str_to_key;
use crate::KEYMAP_FILE;

#[derive(Debug)]
pub struct JoshutoCommandMapping {
    map: HashMap<Key, CommandKeybind>,
}

impl std::convert::AsRef<HashMap<Key, CommandKeybind>> for JoshutoCommandMapping {
    fn as_ref(&self) -> &HashMap<Key, CommandKeybind> {
        &self.map
    }
}

impl std::convert::AsMut<HashMap<Key, CommandKeybind>> for JoshutoCommandMapping {
    fn as_mut(&mut self) -> &mut HashMap<Key, CommandKeybind> {
        &mut self.map
    }
}

impl JoshutoCommandMapping {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn default_res(&mut self) -> Result<(), String> {
        let mut m = self;

        let cmd = KeyCommand::CursorMoveUp(1);
        let keys = [Key::Up];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::CursorMoveDown(1);
        let keys = [Key::Down];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::ParentDirectory;
        let keys = [Key::Left];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::OpenFile;
        let keys = [Key::Right];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::OpenFile;
        let keys = [Key::Char('\n')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CursorMoveHome;
        let keys = [Key::Home];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CursorMoveEnd;
        let keys = [Key::End];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CursorMovePageUp;
        let keys = [Key::PageUp];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CursorMovePageDown;
        let keys = [Key::PageDown];
        insert_keycommand(&mut m, cmd, &keys)?;

        // vim keys
        let cmd = KeyCommand::CursorMoveUp(1);
        let keys = [Key::Char('k')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CursorMoveDown(1);
        let keys = [Key::Char('j')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::ParentDirectory;
        let keys = [Key::Char('h')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::OpenFile;
        let keys = [Key::Char('l')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::NewTab;
        let keys = [Key::Char('T')];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::NewTab;
        let keys = [Key::Ctrl('t')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CloseTab;
        let keys = [Key::Char('W')];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::CloseTab;
        let keys = [Key::Ctrl('w')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CloseTab;
        let keys = [Key::Char('q')];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::ForceQuit;
        let keys = [Key::Char('Q')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::ReloadDirList;
        let keys = [Key::Char('R')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::ToggleHiddenFiles;
        let keys = [Key::Char('z'), Key::Char('h')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::TabSwitch(1);
        let keys = [Key::Char('\t')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::TabSwitch(-1);
        let keys = [Key::BackTab];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::OpenFileWith;
        let keys = [Key::Char('r')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CutFiles;
        let keys = [Key::Char('d'), Key::Char('d')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CopyFiles;
        let keys = [Key::Char('y'), Key::Char('y')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::PasteFiles(IOWorkerOptions::default());
        let keys = [Key::Char('p'), Key::Char('p')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::DeleteFiles;
        let keys = [Key::Delete];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::DeleteFiles;
        let keys = [Key::Char('D'), Key::Char('d')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::RenameFileAppend;
        let keys = [Key::Char('a')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::RenameFilePrepend;
        let keys = [Key::Char('A')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CommandLine("search ".to_string(), "".to_string());
        let keys = [Key::Char('/')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::SearchNext;
        let keys = [Key::Char('n')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::SearchPrev;
        let keys = [Key::Char('N')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::BulkRename;
        let keys = [Key::Char('b'), Key::Char('b')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::SetMode;
        let keys = [Key::Char('=')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CommandLine("".to_string(), "".to_string());
        let keys = [Key::Char(';')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CommandLine("mkdir ".to_string(), "".to_string());
        let keys = [Key::Char('m'), Key::Char('k')];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CommandLine("rename ".to_string(), "".to_string());
        let keys = [Key::Char('c'), Key::Char('w')];
        insert_keycommand(&mut m, cmd, &keys)?;

        Ok(())
    }
}

impl std::default::Default for JoshutoCommandMapping {
    fn default() -> Self {
        let mut m = Self {
            map: HashMap::new(),
        };

        let _ = m.default_res();

        m
    }
}

impl ConfigStructure for JoshutoCommandMapping {
    fn get_config() -> Self {
        parse_to_config_file::<JoshutoRawCommandMapping, JoshutoCommandMapping>(KEYMAP_FILE)
            .unwrap_or_else(Self::default)
    }
}

#[derive(Debug, Deserialize)]
struct JoshutoMapCommand {
    pub command: String,
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
            match KeyCommand::parse_command(m.command.as_str()) {
                Ok(command) => {
                    let keycodes: Vec<Key> = m
                        .keys
                        .iter()
                        .filter_map(|s| str_to_key(s.as_str()))
                        .collect();

                    if keycodes.len() != m.keys.len() {
                        eprintln!("Failed to parse keycodes: {:?}", m.keys);
                        continue;
                    }

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

fn insert_keycommand(
    keymap: &mut JoshutoCommandMapping,
    keycommand: KeyCommand,
    keycodes: &[Key],
) -> Result<(), String> {
    let keycode_len = keycodes.len();

    if keycode_len == 0 {
        return Ok(());
    }

    let key = keycodes[0];

    if keycode_len == 1 {
        match keymap.as_mut().entry(key) {
            Entry::Occupied(_) => {
                return Err(format!("Error: Keybindings ambiguous for {}", keycommand))
            }
            Entry::Vacant(entry) => entry.insert(CommandKeybind::SimpleKeybind(keycommand)),
        };
        return Ok(());
    }

    match keymap.as_mut().entry(key) {
        Entry::Occupied(mut entry) => match entry.get_mut() {
            CommandKeybind::CompositeKeybind(ref mut m) => {
                insert_keycommand(m, keycommand, &keycodes[1..])
            }
            _ => Err(format!("Error: Keybindings ambiguous for {}", keycommand)),
        },
        Entry::Vacant(entry) => {
            let mut new_map = JoshutoCommandMapping::new();
            let result = insert_keycommand(&mut new_map, keycommand, &keycodes[1..]);
            if result.is_ok() {
                let composite_command = CommandKeybind::CompositeKeybind(new_map);
                entry.insert(composite_command);
            }
            result
        }
    }
}
