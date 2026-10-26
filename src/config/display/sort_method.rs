use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

/// A single criterion to sort directory entries by.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum SortMethod {
    #[default]
    #[serde(rename = "lexical")]
    Lexical,
    #[serde(rename = "mtime")]
    Mtime,
    #[serde(rename = "natural")]
    Natural,
    #[serde(rename = "size")]
    Size,
    #[serde(rename = "exit")]
    Ext,
}

impl SortMethod {
    /// Parses a sort method name (e.g. `"lexical"`, `"mtime"`) from a command/config string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "lexical" => Some(SortMethod::Lexical),
            "mtime" => Some(SortMethod::Mtime),
            "natural" => Some(SortMethod::Natural),
            "size" => Some(SortMethod::Size),
            "ext" => Some(SortMethod::Ext),
            _ => None,
        }
    }
    /// Returns the sort method's config/display name.
    pub const fn as_str(&self) -> &str {
        match *self {
            SortMethod::Lexical => "lexical",
            SortMethod::Mtime => "mtime",
            SortMethod::Natural => "natural",
            SortMethod::Size => "size",
            SortMethod::Ext => "ext",
        }
    }
}

impl std::fmt::Display for SortMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A priority-ordered list of sort methods, used as tiebreakers when comparing entries: the
/// front of the list wins, with the most-recently-selected method moved to the front.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SortMethodList {
    pub list: VecDeque<SortMethod>,
}

impl SortMethodList {
    /// Moves `st` to the front of the priority list, dropping the lowest-priority method.
    pub fn reorganize(&mut self, st: SortMethod) {
        self.list.push_front(st);
        self.list.pop_back();
    }
}

impl std::default::Default for SortMethodList {
    fn default() -> Self {
        let list: VecDeque<SortMethod> = vec![
            SortMethod::Natural,
            SortMethod::Lexical,
            SortMethod::Size,
            SortMethod::Ext,
            SortMethod::Mtime,
        ]
        .into_iter()
        .collect();

        Self { list }
    }
}
