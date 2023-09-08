use serde::{Deserialize, Serialize};

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
