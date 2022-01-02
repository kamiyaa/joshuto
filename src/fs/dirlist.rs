use std::cmp;
use std::path;
use std::slice::{Iter, IterMut};

use crate::config::option::DisplayOption;
use crate::context::UiContext;
use crate::fs::{JoshutoDirEntry, JoshutoMetadata};
use crate::history::read_directory;

#[derive(Clone, Debug)]
pub struct JoshutoDirList {
    path: path::PathBuf,
    pub contents: Vec<JoshutoDirEntry>,
    pub metadata: JoshutoMetadata,
    /// The cursor position in this dir list
    index: Option<usize>,
    /// The index in this dir list to start with when rendering the list
    viewport_index: usize,
    _need_update: bool,
}

impl JoshutoDirList {
    pub fn new(
        path: path::PathBuf,
        contents: Vec<JoshutoDirEntry>,
        index: Option<usize>,
        viewport_index: usize,
        metadata: JoshutoMetadata,
    ) -> Self {
        Self {
            path,
            contents,
            metadata,
            index,
            viewport_index,
            _need_update: false,
        }
    }

    pub fn from_path(path: path::PathBuf, options: &DisplayOption) -> std::io::Result<Self> {
        let filter_func = options.filter_func();
        let mut contents = read_directory(path.as_path(), filter_func, options)?;

        let sort_options = options.sort_options_ref();
        contents.sort_by(|f1, f2| sort_options.compare(f1, f2));

        let index = if contents.is_empty() { None } else { Some(0) };

        let metadata = JoshutoMetadata::from(&path)?;

        Ok(Self {
            path,
            contents,
            metadata,
            _need_update: false,
            index,
            viewport_index: if let Some(ix) = index { ix } else { 0 },
        })
    }

    pub fn get_index(&self) -> Option<usize> {
        self.index
    }

    fn update_viewport(&mut self, ui_context: &UiContext, options: &DisplayOption) {
        if let Some(ix) = self.index {
            let height = ui_context.layout[0].height as usize;

            // get scroll buffer size, corrected in case of too small terminal
            let scroll_offset = if height < 4 {
                0
            } else if options.scroll_offset() * 2 > height - 1 {
                height / 2 - 1
            } else {
                options.scroll_offset()
            };

            // calculate viewport
            let viewport_end = self.viewport_index + height;
            if (viewport_end as i16 - ix as i16 - 1) < scroll_offset as i16 {
                // cursor too low
                self.viewport_index = (ix + scroll_offset - height + 1) as usize;
            } else if (ix as i16 - self.viewport_index as i16) < scroll_offset as i16 {
                // cursor too high
                self.viewport_index = cmp::max(ix as i16 - scroll_offset as i16, 0) as usize;
            }
        } else {
            self.viewport_index = 0;
        }
    }

    pub fn set_index(
        &mut self,
        index: Option<usize>,
        ui_context: &UiContext,
        options: &DisplayOption,
    ) {
        if index == self.index {
            return;
        }
        self.index = index;
        if ui_context.layout.len() != 0 {
            self.update_viewport(ui_context, options);
        }
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
        self._need_update = true;
    }

    pub fn need_update(&self) -> bool {
        self._need_update || self.modified()
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

    /// Returns the index of the first entry to be printed in a UI dir list
    pub fn first_index_for_viewport(&self) -> usize {
        self.viewport_index
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
