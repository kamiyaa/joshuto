use std::collections::HashMap;

use ratatui::termion::event::Event;
use serde::{Deserialize, Serialize};

use crate::{traits::config::TomlConfigFile, types::config_type::ConfigType, utils::keyparse};

pub type Bookmarks = HashMap<Event, String>;

impl TomlConfigFile for Bookmarks {
    type Raw = BookmarksRaw;

    fn get_type() -> ConfigType {
        ConfigType::Bookmarks
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookmarkRaw {
    pub key: String,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookmarksRaw {
    #[serde(default)]
    pub bookmark: Vec<BookmarkRaw>,
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
