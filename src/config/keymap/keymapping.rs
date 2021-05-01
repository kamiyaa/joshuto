use serde_derive::Deserialize;

use std::collections::{hash_map::Entry, HashMap};

#[cfg(feature = "mouse")]
use termion::event::MouseEvent;
use termion::event::{Event, Key};

use super::keyparse::str_to_event;
use crate::commands::{CommandKeybind, KeyCommand};
use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::io::IoWorkerOptions;

#[derive(Debug, Deserialize)]
struct CommandKeymap {
    pub command: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawAppKeyMapping {
    #[serde(default)]
    mapcommand: Vec<CommandKeymap>,
}

impl Flattenable<AppKeyMapping> for RawAppKeyMapping {
    fn flatten(self) -> AppKeyMapping {
        let mut keymaps = AppKeyMapping::new();
        for m in self.mapcommand {
            match KeyCommand::parse_command(m.command.as_str()) {
                Ok(command) => {
                    let events: Vec<Event> = m
                        .keys
                        .iter()
                        .filter_map(|s| str_to_event(s.as_str()))
                        .collect();

                    if events.len() != m.keys.len() {
                        eprintln!("Failed to parse events: {:?}", m.keys);
                        continue;
                    }

                    let result = insert_keycommand(&mut keymaps, command, &events);
                    match result {
                        Ok(_) => {}
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        keymaps
    }
}

#[derive(Debug)]
pub struct AppKeyMapping {
    map: HashMap<Event, CommandKeybind>,
}

impl std::convert::AsRef<HashMap<Event, CommandKeybind>> for AppKeyMapping {
    fn as_ref(&self) -> &HashMap<Event, CommandKeybind> {
        &self.map
    }
}

impl std::convert::AsMut<HashMap<Event, CommandKeybind>> for AppKeyMapping {
    fn as_mut(&mut self) -> &mut HashMap<Event, CommandKeybind> {
        &mut self.map
    }
}

impl AppKeyMapping {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn default_res(&mut self) -> Result<(), String> {
        let mut m = self;

        let cmd = KeyCommand::CursorMoveUp(1);
        let keys = [Event::Key(Key::Up)];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::CursorMoveDown(1);
        let keys = [Event::Key(Key::Down)];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::ParentDirectory;
        let keys = [Event::Key(Key::Left)];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::OpenFile;
        let keys = [Event::Key(Key::Right)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::OpenFile;
        let keys = [Event::Key(Key::Char('\n'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CursorMoveHome;
        let keys = [Event::Key(Key::Home)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CursorMoveEnd;
        let keys = [Event::Key(Key::End)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CursorMovePageUp;
        let keys = [Event::Key(Key::PageUp)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CursorMovePageDown;
        let keys = [Event::Key(Key::PageDown)];
        insert_keycommand(&mut m, cmd, &keys)?;

        // vim keys
        let cmd = KeyCommand::CursorMoveUp(1);
        let keys = [Event::Key(Key::Char('k'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CursorMoveDown(1);
        let keys = [Event::Key(Key::Char('j'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::ParentDirectory;
        let keys = [Event::Key(Key::Char('h'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::OpenFile;
        let keys = [Event::Key(Key::Char('l'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::NewTab;
        let keys = [Event::Key(Key::Char('T'))];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::NewTab;
        let keys = [Event::Key(Key::Ctrl('t'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CloseTab;
        let keys = [Event::Key(Key::Char('W'))];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::CloseTab;
        let keys = [Event::Key(Key::Ctrl('w'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CloseTab;
        let keys = [Event::Key(Key::Char('q'))];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = KeyCommand::ForceQuit;
        let keys = [Event::Key(Key::Char('Q'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::ReloadDirList;
        let keys = [Event::Key(Key::Char('R'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::ToggleHiddenFiles;
        let keys = [Event::Key(Key::Char('z')), Event::Key(Key::Char('h'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::TabSwitch(1);
        let keys = [Event::Key(Key::Char('\t'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::TabSwitch(-1);
        let keys = [Event::Key(Key::BackTab)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::OpenFileWith;
        let keys = [Event::Key(Key::Char('r'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CutFiles;
        let keys = [Event::Key(Key::Char('d')), Event::Key(Key::Char('d'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CopyFiles;
        let keys = [Event::Key(Key::Char('y')), Event::Key(Key::Char('y'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::PasteFiles(IoWorkerOptions::default());
        let keys = [Event::Key(Key::Char('p')), Event::Key(Key::Char('p'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::DeleteFiles;
        let keys = [Event::Key(Key::Delete)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::DeleteFiles;
        let keys = [Event::Key(Key::Char('D')), Event::Key(Key::Char('d'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::RenameFileAppend;
        let keys = [Event::Key(Key::Char('a'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::RenameFilePrepend;
        let keys = [Event::Key(Key::Char('A'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CommandLine("search ".to_string(), "".to_string());
        let keys = [Event::Key(Key::Char('/'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::SearchNext;
        let keys = [Event::Key(Key::Char('n'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::SearchPrev;
        let keys = [Event::Key(Key::Char('N'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::BulkRename;
        let keys = [Event::Key(Key::Char('b')), Event::Key(Key::Char('b'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::SetMode;
        let keys = [Event::Key(Key::Char('='))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CommandLine("".to_string(), "".to_string());
        let keys = [Event::Key(Key::Char(';'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CommandLine("mkdir ".to_string(), "".to_string());
        let keys = [Event::Key(Key::Char('m')), Event::Key(Key::Char('k'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = KeyCommand::CommandLine("rename ".to_string(), "".to_string());
        let keys = [Event::Key(Key::Char('c')), Event::Key(Key::Char('w'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        Ok(())
    }
}

impl std::default::Default for AppKeyMapping {
    fn default() -> Self {
        let mut m = Self {
            map: HashMap::new(),
        };
        let _ = m.default_res();
        m
    }
}

impl ConfigStructure for AppKeyMapping {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<RawAppKeyMapping, AppKeyMapping>(file_name)
            .unwrap_or_else(Self::default)
    }
}

fn insert_keycommand(
    keymap: &mut AppKeyMapping,
    keycommand: KeyCommand,
    events: &[Event],
) -> Result<(), String> {
    let num_events = events.len();
    if num_events == 0 {
        return Ok(());
    }

    let event = events[0].clone();
    if num_events == 1 {
        match keymap.as_mut().entry(event) {
            Entry::Occupied(_) => {
                return Err(format!("Error: Keybindings ambiguous for {}", keycommand))
            }
            Entry::Vacant(entry) => entry.insert(CommandKeybind::SimpleKeybind(keycommand)),
        };
        return Ok(());
    }

    match keymap.as_mut().entry(event) {
        Entry::Occupied(mut entry) => match entry.get_mut() {
            CommandKeybind::CompositeKeybind(ref mut m) => {
                insert_keycommand(m, keycommand, &events[1..])
            }
            _ => Err(format!("Error: Keybindings ambiguous for {}", keycommand)),
        },
        Entry::Vacant(entry) => {
            let mut new_map = AppKeyMapping::new();
            let result = insert_keycommand(&mut new_map, keycommand, &events[1..]);
            if result.is_ok() {
                let composite_command = CommandKeybind::CompositeKeybind(new_map);
                entry.insert(composite_command);
            }
            result
        }
    }
}
