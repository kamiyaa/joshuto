use std::{collections::HashMap, path::PathBuf};

use crate::fs::DirListDisplayOptions;
use crate::types::option::line_mode::LineMode;
use crate::types::option::sort::SortOption;

/// Display options valid per JoshutoTab
#[derive(Clone, Debug, Default)]
pub struct TabDisplayOption {
    pub dirlist_options: HashMap<PathBuf, DirListDisplayOptions>,
    pub sort_options: SortOption,
    pub linemode: LineMode,
}

impl TabDisplayOption {
    /// Returns the current sort options for this tab.
    pub fn sort_options_ref(&self) -> &SortOption {
        &self.sort_options
    }

    /// Returns a mutable reference to this tab's sort options.
    pub fn sort_options_mut(&mut self) -> &mut SortOption {
        &mut self.sort_options
    }

    /// Returns the per-directory display options for `path`, if set.
    pub fn dirlist_options_ref(&self, path: &PathBuf) -> Option<&DirListDisplayOptions> {
        self.dirlist_options.get(path)
    }

    /// Returns a mutable reference to `path`'s display options, inserting the default if unset.
    pub fn dirlist_options_mut(&mut self, path: &PathBuf) -> &mut DirListDisplayOptions {
        if !self.dirlist_options.contains_key(path) {
            self.dirlist_options
                .insert(path.to_owned(), Default::default());
        }
        self.dirlist_options.get_mut(path).unwrap()
    }
}
