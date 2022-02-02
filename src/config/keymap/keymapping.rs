use serde_derive::Deserialize;

use std::collections::{hash_map::Entry, HashMap};
use std::convert::{AsMut, AsRef, From};
use std::str::FromStr;

use termion::event::Event;

use crate::config::{parse_to_config_file, TomlConfigFile};
use crate::error::JoshutoResult;
use crate::key_command::{Command, CommandKeybind};
use crate::util::keyparse::str_to_event;

use super::DEFAULT_CONFIG_FILE_PATH;

#[derive(Debug, Deserialize)]
struct CommandKeymap {
    pub command: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AppKeyMappingCrude {
    #[serde(default)]
    pub mapcommand: Vec<CommandKeymap>,
}

#[derive(Debug)]
pub struct AppKeyMapping {
    map: HashMap<Event, CommandKeybind>,
}

impl AppKeyMapping {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn default_res() -> JoshutoResult<Self> {
        let crude: AppKeyMappingCrude = toml::from_str(DEFAULT_CONFIG_FILE_PATH)?;
        let keymapping: Self = Self::from(crude);
        Ok(keymapping)
    }
}

impl AsRef<HashMap<Event, CommandKeybind>> for AppKeyMapping {
    fn as_ref(&self) -> &HashMap<Event, CommandKeybind> {
        &self.map
    }
}

impl AsMut<HashMap<Event, CommandKeybind>> for AppKeyMapping {
    fn as_mut(&mut self) -> &mut HashMap<Event, CommandKeybind> {
        &mut self.map
    }
}

impl From<AppKeyMappingCrude> for AppKeyMapping {
    fn from(crude: AppKeyMappingCrude) -> Self {
        let mut keymaps = Self::new();
        for m in crude.mapcommand {
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

impl TomlConfigFile for AppKeyMapping {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<AppKeyMappingCrude, AppKeyMapping>(file_name).unwrap_or_else(|| {
            eprintln!("Using default keymapping");
            Self::default()
        })
    }
}

impl std::default::Default for AppKeyMapping {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        AppKeyMapping::default_res().unwrap()
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
