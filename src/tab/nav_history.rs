use std::path::{Path, PathBuf};

/// Back/forward navigation history for a tab, similar to a browser's history stack.
#[derive(Default)]
pub struct NavigationHistory {
    items: Vec<PathBuf>,
    index: usize,
}

impl From<&PathBuf> for NavigationHistory {
    fn from(value: &PathBuf) -> Self {
        Self {
            items: vec![value.to_path_buf()],
            index: 0,
        }
    }
}

impl NavigationHistory {
    /// Moves back one entry in the history, returning the new current path, or `None` if
    /// already at the oldest entry.
    pub fn prev(&mut self) -> Option<&PathBuf> {
        if self.index == 0 {
            return None;
        }

        self.index -= 1;
        self.items.get(self.index)
    }

    /// Moves forward one entry in the history, returning the new current path, or `None` if
    /// already at the newest entry.
    pub fn next(&mut self) -> Option<&PathBuf> {
        if self.index == self.items.len() - 1 {
            return None;
        }

        self.index += 1;
        self.items.get(self.index)
    }

    /// Pushes `path` as the new current entry, discarding any forward (redo) history.
    pub fn push(&mut self, path: &Path) {
        self.index += 1;

        if self.index < self.items.len() {
            self.items.truncate(self.index);
        }

        self.items.push(path.to_path_buf());
    }

    /// Removes the current entry from the history.
    pub fn remove_current(&mut self) {
        self.items.remove(self.index);
        self.items.dedup();
    }
}
