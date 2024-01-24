use termion::event::Event;

use std::collections::HashMap;

use crate::config::raw::bookmarks::BookmarksRaw;
use crate::config::{ConfigType, TomlConfigFile};
use crate::util::keyparse;

pub type Bookmarks = HashMap<Event, String>;

impl TomlConfigFile for Bookmarks {
    type Raw = BookmarksRaw;

    fn get_type() -> ConfigType {
        ConfigType::Bookmarks
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
