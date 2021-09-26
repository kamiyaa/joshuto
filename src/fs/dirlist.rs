use std::path;
use std::slice::{Iter, IterMut};

use crate::fs::{JoshutoDirEntry, JoshutoMetadata};
use crate::history::read_directory;
use crate::util::display::DisplayOption;

#[derive(Clone, Debug)]
pub struct JoshutoDirList {
    pub index: Option<usize>,
    path: path::PathBuf,
    content_outdated: bool,
    pub metadata: JoshutoMetadata,
    pub contents: Vec<JoshutoDirEntry>,
}

impl JoshutoDirList {
    pub fn new(
        path: path::PathBuf,
        contents: Vec<JoshutoDirEntry>,
        index: Option<usize>,
        metadata: JoshutoMetadata,
    ) -> Self {
        Self {
            index,
            path,
            content_outdated: false,
            metadata,
            contents,
        }
    }

    pub fn from_path(path: path::PathBuf, options: &DisplayOption) -> std::io::Result<Self> {
        let filter_func = options.filter_func();
        let mut contents = read_directory(path.as_path(), filter_func, &options)?;

        let sort_options = options.sort_options_ref();
        contents.sort_by(|f1, f2| sort_options.compare(f1, f2));

        let index = if contents.is_empty() { None } else { Some(0) };

        let metadata = JoshutoMetadata::from(&path)?;

        Ok(Self {
            index,
            path,
            content_outdated: false,
            metadata,
            contents,
        })
    }

    pub fn iter(&self) -> Iter<JoshutoDirEntry> {
        self.contents.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<JoshutoDirEntry> {
        self.contents.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn modified(&self) -> bool {
        let metadata = std::fs::symlink_metadata(self.file_path());
        match metadata {
            Ok(m) => match m.modified() {
                Ok(s) => s > self.metadata.modified(),
                _ => false,
            },
            _ => false,
        }
    }

    pub fn depreciate(&mut self) {
        self.content_outdated = true;
    }

    pub fn need_update(&self) -> bool {
        self.content_outdated || self.modified()
    }

    pub fn file_path(&self) -> &path::PathBuf {
        &self.path
    }

    pub fn any_selected(&self) -> bool {
        self.contents.iter().any(|e| e.is_selected())
    }

    pub fn iter_selected(&self) -> impl Iterator<Item = &JoshutoDirEntry> {
        self.contents.iter().filter(|entry| entry.is_selected())
    }

    pub fn iter_selected_mut(&mut self) -> impl Iterator<Item = &mut JoshutoDirEntry> {
        self.contents.iter_mut().filter(|entry| entry.is_selected())
    }

    pub fn get_selected_paths(&self) -> Vec<path::PathBuf> {
        let vec: Vec<path::PathBuf> = self
            .iter_selected()
            .map(|e| e.file_path().to_path_buf())
            .collect();
        if !vec.is_empty() {
            vec
        } else {
            match self.curr_entry_ref() {
                Some(s) => vec![s.file_path().to_path_buf()],
                _ => vec![],
            }
        }
    }

    pub fn curr_entry_ref(&self) -> Option<&JoshutoDirEntry> {
        self.get_curr_ref_(self.index?)
    }

    pub fn curr_entry_mut(&mut self) -> Option<&mut JoshutoDirEntry> {
        self.get_curr_mut_(self.index?)
    }

    /// For a given number of entries, visible in a UI, this method returns the index of the entry
    /// with which the UI should start to list the entries.
    ///
    /// This method assures that the cursor is always in the viewport of the UI.
    pub fn first_index_for_viewport(&self, viewport_height: usize) -> usize {
        match self.index {
            Some(index) => index / viewport_height as usize * viewport_height as usize,
            None => 0,
        }
    }

    fn get_curr_mut_(&mut self, index: usize) -> Option<&mut JoshutoDirEntry> {
        if index < self.contents.len() {
            Some(&mut self.contents[index])
        } else {
            None
        }
    }

    fn get_curr_ref_(&self, index: usize) -> Option<&JoshutoDirEntry> {
        if index < self.contents.len() {
            Some(&self.contents[index])
        } else {
            None
        }
    }
}
