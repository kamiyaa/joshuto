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
    fn pop_or_create(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> std::io::Result<JoshutoDirList>;
    fn get_mut_or_create(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> std::io::Result<&mut JoshutoDirList>;
    fn depreciate_all_entries(&mut self);
}

pub type JoshutoHistory = HashMap<PathBuf, JoshutoDirList>;

impl DirectoryHistory for JoshutoHistory {
    fn populate_to_root(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> std::io::Result<()> {
        let mut ancestors = path.ancestors();
        if let Some(mut ancestor) = ancestors.next() {
            for curr in ancestors {
                match self.entry(curr.to_path_buf()) {
                    Entry::Occupied(mut entry) => {
                        let dirlist = entry.get_mut();
                        dirlist.reload_contents(sort_option)?;
                        if let Some(i) = get_index_of_value(&dirlist.contents, &ancestor) {
                            dirlist.index = Some(i);
                        }
                    }
                    Entry::Vacant(entry) => {
                        let mut dirlist =
                            JoshutoDirList::new(curr.to_path_buf().clone(), sort_option)?;
                        if let Some(i) = get_index_of_value(&dirlist.contents, &ancestor) {
                            dirlist.index = Some(i);
                        }
                        entry.insert(dirlist);
                    }
                }
                ancestor = curr;
            }
        }
        Ok(())
    }

    fn pop_or_create(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> std::io::Result<JoshutoDirList> {
        match self.remove(&path.to_path_buf()) {
            Some(mut dirlist) => {
                if dirlist.need_update() {
                    dirlist.reload_contents(&sort_option)?
                } else {
                    let metadata = std::fs::symlink_metadata(dirlist.file_path())?;

                    let modified = metadata.modified()?;
                    if modified > dirlist.metadata.modified {
                        dirlist.reload_contents(&sort_option)?
                    }
                }
                Ok(dirlist)
            }
            None => {
                let path_clone = path.to_path_buf();
                let dirlist = JoshutoDirList::new(path_clone, &sort_option)?;
                Ok(dirlist)
            }
        }
    }
    fn get_mut_or_create(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> std::io::Result<&mut JoshutoDirList> {
        match self.entry(path.to_path_buf().clone()) {
            Entry::Occupied(entry) => {
                /*
                                if dir_entry.need_update() {
                                    dir_entry.reload_contents(&sort_option)?;
                                }
                */
                Ok(entry.into_mut())
            }
            Entry::Vacant(entry) => {
                let s = JoshutoDirList::new(path.to_path_buf(), &sort_option)?;
                Ok(entry.insert(s))
            }
        }
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
