use std::collections::{hash_map::Entry, HashMap};
use std::path::PathBuf;
use termion::event::{Event, Key};

use crate::error::JoshutoResult;
use crate::error::{JoshutoError, JoshutoErrorKind};
type P = PathBuf;

#[derive(Debug, Clone)]
pub struct AppBookmarkMapping {
    pub map: HashMap<Event, P>,
}

impl AppBookmarkMapping {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(Event::Key(Key::Char('h')), P::from("/home/mg"));
        map.insert(Event::Key(Key::Char('c')), P::from("/home/mg/.config"));
        map.insert(Event::Key(Key::Char('o')), P::from("/home/mg/HOMEDATA"));

        Self { map }
    }
}

pub fn insert_bookmark(
    keymap: &mut AppBookmarkMapping,
    path: P,
    event: Event,
) -> JoshutoResult<()> {
    match keymap.map.entry(event) {
        Entry::Occupied(_) => {
            return Err(JoshutoError::new(
                JoshutoErrorKind::EnvVarNotPresent,
                format!(" BLABLA"),
            ))
        }
        Entry::Vacant(entry) => entry.insert(path),
    };
    return Ok(());
}
