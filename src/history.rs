use std::collections::{hash_map::Entry, HashMap};
use std::path::{Path, PathBuf};

use crate::sort;
use crate::structs::JoshutoDirList;

pub trait DirectoryHistory {
    fn populate_to_root(&mut self, pathbuf: &PathBuf, sort_option: &sort::SortOption);
    fn pop_or_create(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> Result<JoshutoDirList, std::io::Error>;
    fn get_mut_or_create(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> Result<&mut JoshutoDirList, std::io::Error>;
    fn depreciate_all_entries(&mut self);
}

pub type JoshutoHistory = HashMap<PathBuf, JoshutoDirList>;

impl DirectoryHistory for JoshutoHistory {
    fn populate_to_root(&mut self, pathbuf: &PathBuf, sort_option: &sort::SortOption) {
        let mut ancestors = pathbuf.ancestors();
        match ancestors.next() {
            None => {}
            Some(mut ancestor) => {
                for curr in ancestors {
                    match JoshutoDirList::new(curr.to_path_buf().clone(), sort_option) {
                        Ok(mut s) => {
                            let index = s.contents.iter().enumerate().find_map(|(i, dir)| {
                                if dir.path == ancestor {
                                    Some(i)
                                } else {
                                    None
                                }
                            });
                            if let Some(i) = index {
                                s.index = Some(i);
                            }
                            self.insert(curr.to_path_buf(), s);
                        }
                        Err(e) => eprintln!("populate_to_root: {}", e),
                    }
                    ancestor = curr;
                }
            }
        }
    }

    fn pop_or_create(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> Result<JoshutoDirList, std::io::Error> {
        match self.remove(&path.to_path_buf()) {
            Some(mut dirlist) => {
                if dirlist.need_update() {
                    dirlist.update_contents(&sort_option)?
                } else {
                    let metadata = std::fs::symlink_metadata(&dirlist.path)?;

                    let modified = metadata.modified()?;
                    if modified > dirlist.metadata.modified {
                        dirlist.update_contents(&sort_option)?
                    }
                }
                Ok(dirlist)
            }
            None => {
                let path_clone = path.to_path_buf();
                JoshutoDirList::new(path_clone, &sort_option)
            }
        }
    }
    fn get_mut_or_create(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> Result<&mut JoshutoDirList, std::io::Error> {
        match self.entry(path.to_path_buf().clone()) {
            Entry::Occupied(entry) => {
                /*
                                if dir_entry.need_update() {
                                    dir_entry.update_contents(&sort_option)?;
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
