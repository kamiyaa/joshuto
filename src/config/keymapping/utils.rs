use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

use ratatui::termion::event::Event;

use crate::{
    app_command::AppCommand,
    config::keymapping::{
        app_keybind::{KeyEventAction, KeyMapping},
        app_keymapping_raw::CommandKeymapRaw,
    },
    error::{AppError, AppErrorCode, AppResult},
    utils::keyparse,
};

///
pub fn command_keymaps_vec_to_map(keymaps: &[CommandKeymapRaw]) -> HashMap<Event, KeyEventAction> {
    let mut hashmap = HashMap::new();

    for keymap in keymaps {
        if keymap.commands.is_empty() && keymap.command.is_none() {
            eprintln!("Keymap `commands` cannot be empty");
            continue;
        }
        let commands: Vec<AppCommand> = match &keymap.command {
            Some(command) => vec![command.clone()],
            None => keymap.commands.clone(),
        }
        .iter()
        .filter_map(|cmd_str| match AppCommand::from_str(cmd_str) {
            Ok(s) => Some(s),
            Err(err) => {
                eprintln!("Keymap error: {:?}", err);
                None
            }
        })
        .collect();

        let expected_len = if keymap.command.is_none() {
            keymap.commands.len()
        } else {
            1
        };

        if commands.len() != expected_len {
            eprintln!("Failed to parse commands: {:?}", keymap.commands);
            continue;
        }

        let key_events: Vec<Event> = keymap
            .keys
            .iter()
            .filter_map(|s| keyparse::str_to_event(s.as_str()))
            .collect();

        if key_events.len() != keymap.keys.len() {
            eprintln!("Failed to parse keys: {:?}", keymap.keys);
            continue;
        }

        let command_description = keymap.description.to_owned();
        if let Err(err) =
            insert_keycommand(&mut hashmap, commands, command_description, &key_events)
        {
            let error_msg = "Failed to insert keybinding";
            tracing::error!(?err, ?key_events, "{error_msg}");
            eprintln!("{error_msg} {:?}: {err}", keymap.keys);
        }
    }
    hashmap
}

fn insert_keycommand(
    keymap: &mut KeyMapping,
    commands: Vec<AppCommand>,
    description: Option<String>,
    events: &[Event],
) -> AppResult {
    let num_events = events.len();
    if num_events == 0 {
        return Ok(());
    }

    let event = events[0].clone();
    if num_events == 1 {
        match keymap.entry(event) {
            Entry::Occupied(_) => {
                let error_msg = "Ambiguous keybindings";
                tracing::debug!("{error_msg}");
                return Err(AppError::new(
                    AppErrorCode::AmbiguousKeybinding,
                    error_msg.to_string(),
                ));
            }
            Entry::Vacant(entry) => entry.insert(KeyEventAction::Base {
                commands,
                description,
            }),
        };
        return Ok(());
    }

    match keymap.entry(event) {
        Entry::Occupied(mut entry) => match entry.get_mut() {
            KeyEventAction::Recursive(ref mut m) => {
                insert_keycommand(m, commands, description, &events[1..])
            }
            _ => {
                let error_msg = "Ambiguous keybindings";
                tracing::debug!("{error_msg}");
                return Err(AppError::new(
                    AppErrorCode::AmbiguousKeybinding,
                    error_msg.to_string(),
                ));
            }
        },
        Entry::Vacant(entry) => {
            let mut new_map = KeyMapping::new();
            let result = insert_keycommand(&mut new_map, commands, description, &events[1..]);
            if result.is_ok() {
                let composite_command = KeyEventAction::Recursive(new_map);
                entry.insert(composite_command);
            }
            result
        }
    }
}
