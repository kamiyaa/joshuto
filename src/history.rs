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
                        for (i, dirent) in s.contents.iter().enumerate() {
                            if dirent.path == ancestor {
                                s.index = Some(i);
                                break;
                            }
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
        let pathbuf = path.to_path_buf();
        match self.map.entry(pathbuf.clone()) {
            Entry::Occupied(mut entry) => {
                let dir_entry = entry.get_mut();
                if dir_entry.need_update() {
                    dir_entry.update_contents(&sort_option).unwrap();
                }
            }
            Entry::Vacant(entry) => {
                let s = structs::JoshutoDirList::new(path.to_path_buf(), &sort_option)?;
                entry.insert(s);
            }
        };
        match self.map.get_mut(&pathbuf) {
            Some(s) => Ok(s),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Can't find file",
            )),
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
            .for_each(|(_, v)| v.update_needed = true);
    }
}
