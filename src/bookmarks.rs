use crate::config::Flattenable;
use crate::error::JoshutoResult;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawBookmarkMapping {
    #[serde(default)]
    pub bookmarks: Vec<(char, String)>,
}

impl RawBookmarkMapping {
    fn save(&self, path: &str) -> std::io::Result<()> {
        let s = toml::to_string_pretty(self);
        if let Ok(s) = s {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .unwrap();

            file.write_all(&s.as_bytes()).unwrap();
        }
        Ok(())
    }
}

impl Flattenable<BookmarkMapping> for RawBookmarkMapping {
    fn flatten(self) -> BookmarkMapping {
        let mut bookmarksmap = BookmarkMapping::new();
        for (c, p) in self.bookmarks {
            bookmarksmap.map.insert(c, PathBuf::from(p));
        }
        // notify(&bookmarks);
        bookmarksmap
    }
}

#[derive(Debug, Clone)]
pub struct BookmarkMapping {
    pub map: HashMap<char, PathBuf>,
}

impl BookmarkMapping {
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        self.to_raw().save(path)?;
        Ok(())
    }

    pub fn to_raw(&self) -> RawBookmarkMapping {
        let mut bookmarks = vec![];
        for (k, v) in self.map.iter() {
            bookmarks.push((*k, v.to_str().unwrap().to_string()));
        }
        return RawBookmarkMapping { bookmarks };
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
            match toml::from_str::<RawBookmarkMapping>(&contents) {
                Ok(rabm) => {
                    let res = rabm.flatten();
                    return res;
                }
                Err(_) => {
                    eprintln!("Error parsing  file: {}", bookmarks_file_path);
                }
            }
        }
        Self::new()
    }
}

pub fn insert_bookmark(
    bookmarks: &mut BookmarkMapping,
    path: PathBuf,
    c: char,
) -> JoshutoResult<()> {
    bookmarks.map.insert(c, path);
    return Ok(());
}
