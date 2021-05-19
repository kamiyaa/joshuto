use std::collections::{hash_map::Entry, HashMap};
use std::path::PathBuf;
use termion::event::{Event, Key};

use crate::config::Flattenable;
use crate::error::JoshutoResult;
use crate::error::{JoshutoError, JoshutoErrorKind};
type P = PathBuf;
use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawAppBookmarkMapping {
    #[serde(default)]
    pub tuples: Vec<(char, String)>,
}
impl RawAppBookmarkMapping {
    fn save(&self, path: &str) -> std::io::Result<()> {
        let s = toml::to_string(self);
        if let Ok(s) = s {
            std::fs::write(path, s.to_string())?;
        }
        Ok(())
    }
}

// fn notify<T: std::fmt::Debug>(x: T) {
//     let log = format!("{:?}", &x);
//     let _ = std::process::Command::new("notify-send").arg(&log).status();
//     std::fs::write("/home/mg/.config/joshuto/bookmarkdebug.txt", log);
// }

impl Flattenable<AppBookmarkMapping> for RawAppBookmarkMapping {
    fn flatten(self) -> AppBookmarkMapping {
        let mut bookmarks = AppBookmarkMapping::new();
        for (c, p) in self.tuples {
            bookmarks
                .map
                .insert(Event::Key(Key::Char(c)), PathBuf::from(p));
        }
        // notify(&bookmarks);
        bookmarks
    }
}

#[derive(Debug, Clone)]
pub struct AppBookmarkMapping {
    pub map: HashMap<Event, P>,
}

impl AppBookmarkMapping {
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        self.to_raw().save(path)?;
        Ok(())
    }

    pub fn to_raw(&self) -> RawAppBookmarkMapping {
        let mut tuples = vec![];
        for (k, v) in self.map.iter() {
            if let Event::Key(Key::Char(c)) = k {
                if let Some(p) = v.to_str() {
                    tuples.push((*c, p.to_string()));
                }
            };
        }
        return RawAppBookmarkMapping { tuples };
    }

    pub fn new() -> Self {
        let map = HashMap::new();
        Self { map }
    }

    pub fn load(bookmarks_file_path: &str) -> Self {
        let file_contents = match std::fs::read_to_string(&bookmarks_file_path) {
            Ok(contents) => Some(contents),
            Err(e) => {
                eprintln!("Error reading {} file: {}", bookmarks_file_path, e);
                return Self::new();
            }
        };
        if let Some(contents) = file_contents {
            match toml::from_str::<RawAppBookmarkMapping>(&contents) {
                Ok(rabm) => {
                    let res = rabm.flatten();
                    // notify(&res);
                    return res;
                }
                Err(_) => {
                    eprintln!("Error parsing  file: {}", bookmarks_file_path);
                    return Self::new();
                }
            }
        }

        let mut res = Self::new();
        res.map
            .insert(Event::Key(Key::Char('d')), P::from("/home/mg/DATA"));
        res
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
    // notify(&keymap);
    return Ok(());
}
