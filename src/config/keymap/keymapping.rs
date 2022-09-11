use serde_derive::Deserialize;

use std::collections::{hash_map::Entry, HashMap};
use std::convert::From;
use std::str::FromStr;

use termion::event::Event;

use crate::config::{parse_to_config_file, TomlConfigFile};
use crate::error::JoshutoResult;
use crate::key_command::{AppCommand, Command, CommandKeybind};
use crate::traits::ToString;
use crate::util::keyparse::str_to_event;

use super::DEFAULT_CONFIG_FILE_PATH;

enum KeymapError {
    Conflict,
}

#[derive(Debug, Deserialize)]
struct CommandKeymap {
    pub command: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AppModeKeyMapping {
    #[serde(default)]
    pub keymap: Vec<CommandKeymap>,
}

#[derive(Debug, Deserialize)]
struct AppKeyMappingRaw {
    pub default_view: AppModeKeyMapping,
    pub task_view: AppModeKeyMapping,
    pub help_view: AppModeKeyMapping,
}

pub type KeyMapping = HashMap<Event, CommandKeybind>;

#[derive(Debug)]
pub struct AppKeyMapping {
    pub default_view: KeyMapping,
    pub task_view: KeyMapping,
    pub help_view: KeyMapping,
}

impl AppKeyMapping {
    pub fn new() -> Self {
        Self {
            default_view: KeyMapping::new(),
            task_view: KeyMapping::new(),
            help_view: KeyMapping::new(),
        }
    }

    pub fn default_res() -> JoshutoResult<Self> {
        let crude: AppKeyMappingRaw = toml::from_str(DEFAULT_CONFIG_FILE_PATH)?;
        let keymapping: Self = Self::from(crude);
        Ok(keymapping)
    }
}

fn vec_to_map(vec: &[CommandKeymap]) -> HashMap<Event, CommandKeybind> {
    let mut hashmap = HashMap::new();

    for m in vec {
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

                let command_str = command.command();
                let result = insert_keycommand(&mut hashmap, command, &events);
                match result {
                    Ok(_) => {}
                    Err(e) => match e {
                        KeymapError::Conflict => {
                            let events_str: Vec<String> =
                                events.iter().map(|e| e.to_string()).collect();
                            eprintln!("Error: Ambiguous Keymapping: Multiple commands mapped to key sequence {:?} {}", events_str, command_str);
                        }
                    },
                }
            }
            Err(e) => eprintln!("Keymap error: {}", e),
        }
    }
    hashmap
}

impl From<AppKeyMappingRaw> for AppKeyMapping {
    fn from(raw: AppKeyMappingRaw) -> Self {
        let mut keymaps = Self::new();
        keymaps.default_view = vec_to_map(&raw.default_view.keymap);
        keymaps.task_view = vec_to_map(&raw.task_view.keymap);
        keymaps.help_view = vec_to_map(&raw.help_view.keymap);
        keymaps
    }
}

impl TomlConfigFile for AppKeyMapping {
    fn get_config(file_name: &str) -> Self {
        match parse_to_config_file::<AppKeyMappingRaw, AppKeyMapping>(file_name) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to parse keymap config: {}", e);
                Self::default()
            }
        }
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
    keymap: &mut KeyMapping,
    keycommand: Command,
    events: &[Event],
) -> Result<(), KeymapError> {
    let num_events = events.len();
    if num_events == 0 {
        return Ok(());
    }

    let event = events[0].clone();
    if num_events == 1 {
        match keymap.entry(event) {
            Entry::Occupied(_) => return Err(KeymapError::Conflict),
            Entry::Vacant(entry) => entry.insert(CommandKeybind::SimpleKeybind(keycommand)),
        };
        return Ok(());
    }

    match keymap.entry(event) {
        Entry::Occupied(mut entry) => match entry.get_mut() {
            CommandKeybind::CompositeKeybind(ref mut m) => {
                insert_keycommand(m, keycommand, &events[1..])
            }
            _ => Err(KeymapError::Conflict),
        },
        Entry::Vacant(entry) => {
            let mut new_map = KeyMapping::new();
            let result = insert_keycommand(&mut new_map, keycommand, &events[1..]);
            if result.is_ok() {
                let composite_command = CommandKeybind::CompositeKeybind(new_map);
                entry.insert(composite_command);
            }
            result
        }
    }
}
