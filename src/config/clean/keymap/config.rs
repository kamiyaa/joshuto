use std::collections::{hash_map::Entry, HashMap};
use std::convert::From;
use std::str::FromStr;

use termion::event::Event;

use crate::config::raw::keymap::{AppKeyMappingRaw, CommandKeymap};
use crate::config::{parse_config_or_default, TomlConfigFile};
use crate::error::JoshutoResult;
use crate::key_command::{Command, CommandKeybind};
use crate::traits::ToString;
use crate::util::keyparse::str_to_event;

use super::DEFAULT_CONFIG_FILE_PATH;

pub enum KeymapError {
    Conflict,
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

fn command_keymaps_vec_to_map(keymaps: &[CommandKeymap]) -> HashMap<Event, CommandKeybind> {
    let mut hashmap = HashMap::new();

    for keymap in keymaps {
        if keymap.commands.is_empty() {
            eprintln!("Keymap `commands` cannot be empty");
            continue;
        }
        let commands: Vec<Command> = keymap
            .commands
            .iter()
            .filter_map(|cmd_str| match Command::from_str(cmd_str) {
                Ok(s) => Some(s),
                Err(err) => {
                    eprintln!("Keymap error: {}", err);
                    None
                }
            })
            .collect();

        if commands.len() != keymap.commands.len() {
            eprintln!("Failed to parse commands: {:?}", keymap.commands);
            continue;
        }

        let key_events: Vec<Event> = keymap
            .keys
            .iter()
            .filter_map(|s| str_to_event(s.as_str()))
            .collect();

        if key_events.len() != keymap.keys.len() {
            eprintln!("Failed to parse keys: {:?}", keymap.keys);
            continue;
        }

        let command_description = keymap.description.to_owned();
        if let Err(err) =
            insert_keycommand(&mut hashmap, commands, command_description, &key_events)
        {
            match err {
                KeymapError::Conflict => {
                    let events_str: Vec<String> =
                        key_events.iter().map(|e| e.to_string()).collect();
                    eprintln!("Error: Ambiguous Keymapping: Multiple commands mapped to key sequence {:?} {:?}", events_str, keymap.commands);
                }
            }
        }
    }
    hashmap
}

impl From<AppKeyMappingRaw> for AppKeyMapping {
    fn from(raw: AppKeyMappingRaw) -> Self {
        let mut keymaps = Self::new();
        keymaps.default_view = command_keymaps_vec_to_map(&raw.default_view.keymap);
        keymaps.task_view = command_keymaps_vec_to_map(&raw.task_view.keymap);
        keymaps.help_view = command_keymaps_vec_to_map(&raw.help_view.keymap);
        keymaps
    }
}

impl TomlConfigFile for AppKeyMapping {
    fn get_config(file_name: &str) -> Self {
        parse_config_or_default::<AppKeyMappingRaw, AppKeyMapping>(file_name)
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
    commands: Vec<Command>,
    description: Option<String>,
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
            Entry::Vacant(entry) => entry.insert(CommandKeybind::SimpleKeybind {
                commands,
                description,
            }),
        };
        return Ok(());
    }

    match keymap.entry(event) {
        Entry::Occupied(mut entry) => match entry.get_mut() {
            CommandKeybind::CompositeKeybind(ref mut m) => {
                insert_keycommand(m, commands, description, &events[1..])
            }
            _ => Err(KeymapError::Conflict),
        },
        Entry::Vacant(entry) => {
            let mut new_map = KeyMapping::new();
            let result = insert_keycommand(&mut new_map, commands, description, &events[1..]);
            if result.is_ok() {
                let composite_command = CommandKeybind::CompositeKeybind(new_map);
                entry.insert(composite_command);
            }
            result
        }
    }
}
