use std::collections::HashMap;

use termion::event::Event;

use crate::config::{parse_to_config_file, TomlConfigFile};
use crate::util::keyparse;

use super::bookmarks_raw::BookmarksRaw;

pub type Bookmarks = HashMap<Event, String>;

impl From<BookmarksRaw> for Bookmarks {
    fn from(raw: BookmarksRaw) -> Self {
        let mut raw = raw;
        let map: Bookmarks = raw
            .bookmark
            .drain(..)
            .filter_map(|bookmark| match keyparse::str_to_event(&bookmark.key) {
                Some(event) => Some((event, bookmark.path)),
                None => None,
            })
            .collect();
        map
    }
}

impl TomlConfigFile for Bookmarks {
    fn get_config(file_name: &str) -> Self {
        match parse_to_config_file::<BookmarksRaw, Bookmarks>(file_name) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to parse app config: {}", e);
                Self::default()
            }
        }
    }
}
