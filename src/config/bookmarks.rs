// use serde_derive::Deserialize;
use std::collections::{hash_map::Entry, HashMap};
use std::path::PathBuf;
// #[cfg(feature = "mouse")]
// use termion::event::MouseEvent;
use termion::event::{Event, Key};

// use super::keyparse::str_to_event;
// use crate::commands::{CommandKeybind, KeyCommand};
// use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};
// use crate::io::IoWorkerOptions;


type P = PathBuf;

#[derive(Debug, Clone)]
pub struct AppBookmarkMapping {
    pub map: HashMap<Event, P>,
}

impl AppBookmarkMapping {
    pub fn new() -> Self{
        let mut map = HashMap::new();
        map.insert(Event::Key(Key::Char('h')), P::from("/home/mg"));
        map.insert(Event::Key(Key::Char('c')), P::from("/home/mg/.config"));
        map.insert(Event::Key(Key::Char('o')), P::from("/home/mg/HOMEDATA"));

        Self{map,}

    }
}


fn notify<T: std::fmt::Debug>(x: T){
    let log = format!("{:?}", x);
    let _  = std::process::Command::new("notify-send").arg(log).status();
}

pub fn insert_bookmark(
    keymap: &mut AppBookmarkMapping,
    path: P,
    event: Event,
) -> Result<(), String> {
    match keymap.map.entry(event) {
        Entry::Occupied(_) => {
            return Err(format!("Error: Keybindings ambiguous for {}", 123))
        }
        Entry::Vacant(entry) => entry.insert(path),
    };
    notify(keymap);
    return Ok(());
}










