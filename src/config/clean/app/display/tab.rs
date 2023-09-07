use std::{collections::HashMap, path::PathBuf};

use super::{dirlist::DirListDisplayOptions, line_mode::LineMode, sort::SortOption};

/// Display options valid per JoshutoTab
#[derive(Clone, Debug, Default)]
pub struct TabDisplayOption {
    pub dirlist_options: HashMap<PathBuf, DirListDisplayOptions>,
    pub sort_options: SortOption,
    pub linemode: LineMode,
}

impl TabDisplayOption {
    pub fn sort_options_ref(&self) -> &SortOption {
        &self.sort_options
    }

    pub fn sort_options_mut(&mut self) -> &mut SortOption {
        &mut self.sort_options
    }

    pub fn dirlist_options_ref(&self, path: &PathBuf) -> Option<&DirListDisplayOptions> {
        self.dirlist_options.get(path)
    }

    pub fn dirlist_options_mut(&mut self, path: &PathBuf) -> &mut DirListDisplayOptions {
        if !self.dirlist_options.contains_key(path) {
            self.dirlist_options
                .insert(path.to_owned(), Default::default());
        }
        self.dirlist_options.get_mut(path).unwrap()
    }
}
