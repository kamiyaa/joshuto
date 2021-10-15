use serde_derive::Deserialize;

use std::collections::{hash_map::Entry, HashMap};
use std::str::FromStr;

#[cfg(feature = "mouse")]
use termion::event::MouseEvent;
use termion::event::{Event, Key};

use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::error::JoshutoResult;
use crate::io::IoWorkerOptions;
use crate::key_command::{Command, CommandKeybind};
use crate::util::keyparse::str_to_event;

use super::default_keymap::DEFAULT_KEYMAP;

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

    pub fn default_res() -> JoshutoResult<Self> {
        let raw: RawAppKeyMapping = toml::from_str(DEFAULT_KEYMAP)?;
        let keymapping: Self = raw.flatten();
        Ok(keymapping)
    }
}

impl std::default::Default for AppKeyMapping {
    fn default() -> Self {
        AppKeyMapping::default_res().unwrap()
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
