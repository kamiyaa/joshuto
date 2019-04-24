use std::collections::{hash_map::Entry, HashMap};
use std::path::{Path, PathBuf};

use crate::sort;
use crate::structs;

pub struct DirHistory {
    map: HashMap<PathBuf, structs::JoshutoDirList>,
}

impl DirHistory {
    pub fn new() -> Self {
        DirHistory {
            map: HashMap::new(),
        }
    }

    pub fn populate_to_root(&mut self, pathbuf: &PathBuf, sort_option: &sort::SortOption) {
        let mut ancestors = pathbuf.ancestors();
        if let Some(mut ancestor) = ancestors.next() {
            for curr in ancestors {
                match structs::JoshutoDirList::new(curr.to_path_buf().clone(), sort_option) {
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
                        self.map.insert(curr.to_path_buf(), s);
                    }
                    Err(e) => eprintln!("populate_to_root: {}", e),
                };
                ancestor = curr;
            }
        }
    }

    pub fn pop_or_create(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> Result<structs::JoshutoDirList, std::io::Error> {
        match self.map.remove(&path.to_path_buf()) {
            Some(mut dir_entry) => {
                if dir_entry.need_update() {
                    dir_entry.update_contents(&sort_option)?
                }
                Ok(dir_entry)
            }
            None => {
                let path_clone = path.to_path_buf();
                structs::JoshutoDirList::new(path_clone, &sort_option)
            }
        }
    }

    pub fn get_mut_or_create(
        &mut self,
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> Result<&mut structs::JoshutoDirList, std::io::Error> {
        match self.map.entry(path.to_path_buf().clone()) {
            Entry::Occupied(mut entry) => {
                let dir_entry = entry.get_mut();
                if dir_entry.need_update() {
                    dir_entry.update_contents(&sort_option)?;
                }
                Ok(entry.into_mut())
            }
            Entry::Vacant(entry) => {
                let s = structs::JoshutoDirList::new(path.to_path_buf(), &sort_option)?;
                Ok(entry.insert(s))
            }
        }
    }

    pub fn put_back(&mut self, dirlist: Option<structs::JoshutoDirList>) {
        if let Some(s) = dirlist {
            self.map.insert(s.path.clone(), s);
        }
    }

    pub fn depecrate_all_entries(&mut self) {
        self.map
            .iter_mut()
            .for_each(|(_, v)| v.depreciate());
    }
}
