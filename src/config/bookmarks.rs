use std::collections::{hash_map::Entry, HashMap};
use std::path::PathBuf;
use termion::event::{Event, Key};

use crate::config::Flattenable;
use crate::error::JoshutoResult;
use crate::error::{JoshutoError, JoshutoErrorKind};
type P = PathBuf;
use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, Deserialize)]
#[derive(Serialize)]
pub struct RawAppBookmarkMapping {
    #[serde(default)]
    pub tuples: Vec<(char, PathBuf)>,
}


fn notify<T: std::fmt::Debug>(x: T) {
    let log = format!("{:?}", &x);
    let _ = std::process::Command::new("notify-send").arg(&log).status();
    std::fs::write("/home/mg/.config/joshuto/bookmarkdebug.txt", log);
}

impl Flattenable<AppBookmarkMapping> for RawAppBookmarkMapping {
    fn flatten(self) -> AppBookmarkMapping {
        
        let mut bookmarks = AppBookmarkMapping::new();
        for (c,p) in self.tuples {
            bookmarks.map.insert(Event::Key(Key::Char(c)), p);

        };
        notify(&bookmarks);
        bookmarks
    }
}



#[derive(Debug, Clone)]
pub struct AppBookmarkMapping {
    pub map: HashMap<Event, P>,
}




impl AppBookmarkMapping {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(Event::Key(Key::Char('*')), PathBuf::from("/home/mg/cupypa"));
/*
        let rabm = RawAppBookmarkMapping{
            tuples: vec![('z', PathBuf::from("/mg/home")),
            ('x', PathBuf::from("/mg/home/DATA")),



            ]
        };
        let ser =  toml::to_string(&rabm); 
        if let Ok(ser) = ser{
        std::fs::write("/home/mg/.config/joshuto/bookmarks.toml", ser);
        }
*/
        Self { map }
    }






    pub fn load(bookmarks_file_path: &str) -> Self {
        let file_contents = match std::fs::read_to_string(&bookmarks_file_path) {
            Ok(contents) => Some(contents),
            Err(e) => {
                eprintln!("Error reading {} file: {}", bookmarks_file_path, e);
                return Self::new()
            },
        };
        if let Some(contents) = file_contents{
            match toml::from_str::<RawAppBookmarkMapping>(&contents) {
                Ok(rabm) => {
                    let res = rabm.flatten();
                    // notify(&res);
                    return res
                },
                Err(_) => {
                    eprintln!("Error parsing  file: {}", bookmarks_file_path);
                    return Self::new() 
                },

            }
        }

        let mut res = Self::new();
        res.map.insert(Event::Key(Key::Char('d')), P::from("/home/mg/DATA"));
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
    notify(&keymap);
    return Ok(());
}
