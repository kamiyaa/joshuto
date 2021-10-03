use serde_derive::Deserialize;

use std::collections::{hash_map::Entry, HashMap};
use std::str::FromStr;

#[cfg(feature = "mouse")]
use termion::event::MouseEvent;
use termion::event::{Event, Key};

use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::io::IoWorkerOptions;
use crate::key_command::{Command, CommandKeybind};
use crate::util::keyparse::str_to_event;

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
            match Command::from_str(m.command.as_str()) {
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

        let cmd = Command::CursorMoveUp(1);
        let keys = [Event::Key(Key::Up)];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = Command::CursorMoveDown(1);
        let keys = [Event::Key(Key::Down)];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = Command::ParentDirectory;
        let keys = [Event::Key(Key::Left)];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = Command::OpenFile;
        let keys = [Event::Key(Key::Right)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::OpenFile;
        let keys = [Event::Key(Key::Char('\n'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CursorMoveHome;
        let keys = [Event::Key(Key::Home)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CursorMoveEnd;
        let keys = [Event::Key(Key::End)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CursorMovePageUp;
        let keys = [Event::Key(Key::PageUp)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CursorMovePageDown;
        let keys = [Event::Key(Key::PageDown)];
        insert_keycommand(&mut m, cmd, &keys)?;

        // vim keys
        let cmd = Command::CursorMoveUp(1);
        let keys = [Event::Key(Key::Char('k'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CursorMoveDown(1);
        let keys = [Event::Key(Key::Char('j'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::ParentDirectory;
        let keys = [Event::Key(Key::Char('h'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::OpenFile;
        let keys = [Event::Key(Key::Char('l'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::NewTab;
        let keys = [Event::Key(Key::Char('T'))];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = Command::NewTab;
        let keys = [Event::Key(Key::Ctrl('t'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CloseTab;
        let keys = [Event::Key(Key::Char('W'))];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = Command::CloseTab;
        let keys = [Event::Key(Key::Ctrl('w'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CloseTab;
        let keys = [Event::Key(Key::Char('q'))];
        insert_keycommand(&mut m, cmd, &keys)?;
        let cmd = Command::ForceQuit;
        let keys = [Event::Key(Key::Char('Q'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::ReloadDirList;
        let keys = [Event::Key(Key::Char('R'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::ToggleHiddenFiles;
        let keys = [Event::Key(Key::Char('z')), Event::Key(Key::Char('h'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::TabSwitch(1);
        let keys = [Event::Key(Key::Char('\t'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::TabSwitch(-1);
        let keys = [Event::Key(Key::BackTab)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::OpenFileWith(None);
        let keys = [Event::Key(Key::Char('r'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CutFiles;
        let keys = [Event::Key(Key::Char('d')), Event::Key(Key::Char('d'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CopyFiles;
        let keys = [Event::Key(Key::Char('y')), Event::Key(Key::Char('y'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::PasteFiles(IoWorkerOptions::default());
        let keys = [Event::Key(Key::Char('p')), Event::Key(Key::Char('p'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::DeleteFiles;
        let keys = [Event::Key(Key::Delete)];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::DeleteFiles;
        let keys = [Event::Key(Key::Char('D')), Event::Key(Key::Char('d'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::RenameFileAppend;
        let keys = [Event::Key(Key::Char('a'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::RenameFilePrepend;
        let keys = [Event::Key(Key::Char('A'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CommandLine("search ".to_string(), "".to_string());
        let keys = [Event::Key(Key::Char('/'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::SearchNext;
        let keys = [Event::Key(Key::Char('n'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::SearchPrev;
        let keys = [Event::Key(Key::Char('N'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::BulkRename;
        let keys = [Event::Key(Key::Char('b')), Event::Key(Key::Char('b'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::SetMode;
        let keys = [Event::Key(Key::Char('='))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CommandLine("".to_string(), "".to_string());
        let keys = [Event::Key(Key::Char(';'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CommandLine("".to_string(), "".to_string());
        let keys = [Event::Key(Key::Char(':'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CommandLine("mkdir ".to_string(), "".to_string());
        let keys = [Event::Key(Key::Char('m')), Event::Key(Key::Char('k'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::CommandLine("rename ".to_string(), "".to_string());
        let keys = [Event::Key(Key::Char('c')), Event::Key(Key::Char('w'))];
        insert_keycommand(&mut m, cmd, &keys)?;

        let cmd = Command::Help;
        let keys = [Event::Key(Key::Char('?'))];
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
    keycommand: Command,
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
