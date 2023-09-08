use termion::event::Event;

use std::collections::HashMap;

use crate::config::raw::bookmarks::BookmarksRaw;
use crate::config::TomlConfigFile;
use crate::util::keyparse;

use crate::config::parse_config_or_default;

pub type Bookmarks = HashMap<Event, String>;

impl TomlConfigFile for Bookmarks {
    fn get_config(file_name: &str) -> Self {
        parse_config_or_default::<BookmarksRaw, Bookmarks>(file_name)
    }
}

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
