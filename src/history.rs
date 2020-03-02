use std::collections::{hash_map::Entry, HashMap};
use std::path::{Path, PathBuf};

use crate::fs::{JoshutoDirEntry, JoshutoDirList};
use crate::sort;

pub trait DirectoryHistory {
    fn populate_to_root(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> std::io::Result<()>;
    fn create_or_update(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> std::io::Result<()>;
    fn depreciate_all_entries(&mut self);
}

pub type JoshutoHistory = HashMap<PathBuf, JoshutoDirList>;

impl DirectoryHistory for JoshutoHistory {
    fn populate_to_root(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> std::io::Result<()> {
        let mut prev: Option<&Path> = None;
        for curr in path.ancestors() {
            match self.entry(curr.to_path_buf()) {
                Entry::Occupied(mut entry) => {
                    let dirlist = entry.get_mut();
                    dirlist.reload_contents(sort_option)?;
                    if let Some(ancestor) = prev.as_ref() {
                        if let Some(i) = get_index_of_value(&dirlist.contents, ancestor) {
                            dirlist.index = Some(i);
                        }
                    }
                    prev = Some(curr);
                }
                Entry::Vacant(entry) => {
                    let mut dirlist = JoshutoDirList::new(curr.to_path_buf().clone(), sort_option)?;
                    if let Some(ancestor) = prev.as_ref() {
                        if let Some(i) = get_index_of_value(&dirlist.contents, ancestor) {
                            dirlist.index = Some(i);
                        }
                    }
                    entry.insert(dirlist);
                    prev = Some(curr);
                }
            }
        }
        Ok(())
    }

    fn create_or_update(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> std::io::Result<()> {
        match self.entry(path.to_path_buf()) {
            Entry::Occupied(mut entry) => {
                let dirlist = entry.get_mut();
                dirlist.reload_contents(sort_option)?;
            }
            Entry::Vacant(entry) => {
                let dirlist = JoshutoDirList::new(path.to_path_buf(), sort_option)?;
                entry.insert(dirlist);
            }
        }
        Ok(())
    }

    fn depreciate_all_entries(&mut self) {
        self.iter_mut().for_each(|(_, v)| v.depreciate());
    }
}

fn get_index_of_value(arr: &[JoshutoDirEntry], val: &Path) -> Option<usize> {
    arr.iter().enumerate().find_map(|(i, dir)| {
        if dir.file_path() == val {
            Some(i)
        } else {
            None
        }
    })
}
