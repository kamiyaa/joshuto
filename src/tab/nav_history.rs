use std::path::{Path, PathBuf};

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
    pub fn prev(&mut self) -> Option<&PathBuf> {
        if self.index == 0 {
            return None;
        }

        self.index -= 1;
        self.items.get(self.index)
    }

    pub fn next(&mut self) -> Option<&PathBuf> {
        if self.index == self.items.len() - 1 {
            return None;
        }

        self.index += 1;
        self.items.get(self.index)
    }

    pub fn push(&mut self, path: &Path) {
        self.index += 1;

        if self.index < self.items.len() {
            self.items.truncate(self.index);
        }

        self.items.push(path.to_path_buf());
    }

    pub fn remove_current(&mut self) {
        self.items.remove(self.index);
        self.items.dedup();
    }
}
