
use std;
use std::collections::HashMap;
use std::fs;
use std::path;

use joshuto::structs;
use joshuto::sort;

pub struct History {
    pub map : HashMap<path::PathBuf, structs::JoshutoDirList>,
}

impl History {

    pub fn new() -> Self
    {
        History {
            map: HashMap::new()
        }
    }

    pub fn insert(&mut self, pathbuf: path::PathBuf, dirlist: structs::JoshutoDirList)
    {
        self.map.insert(pathbuf, dirlist);
    }

    pub fn populate_to_root(&mut self, pathbuf: &path::PathBuf,
       sort_type: &sort::SortType)
    {
        let mut pathbuf = pathbuf.clone();

        while pathbuf.parent() != None {
            {
                let parent = pathbuf.parent().unwrap().to_path_buf();
                match structs::JoshutoDirList::new(parent.as_path(), sort_type) {
                    Ok(mut s) => {
                        for (i, dirent) in s.contents.as_ref().unwrap().iter().enumerate() {
                            if dirent.entry.path() == pathbuf {
                                s.index = i as i32;
                                break;
                            }
                        }
                        self.map.insert(parent, s);
                    },
                    Err(e) => { eprintln!("{}", e); }
                };
            }
            if pathbuf.pop() == false {
                break;
            }
        }
    }

    pub fn pop_or_create(&mut self, path : &path::Path,
       sort_type: &sort::SortType)
            -> Result<structs::JoshutoDirList, std::io::Error>
    {
        match self.map.remove(path) {
            Some(mut dir_entry) => {
                let metadata = fs::metadata(&path)?;
                let modified = metadata.modified()?;
                if modified > dir_entry.modified {
                    dir_entry.modified = modified;
                    dir_entry.need_update = true;
                }
                if dir_entry.need_update {
                    dir_entry.update(&path, &sort_type);
                }
                Ok(dir_entry)
            },
            None => {
                structs::JoshutoDirList::new(&path, &sort_type)
            }
        }

    }


    pub fn depecrate_all_entries(&mut self)
    {
        for (_, direntry) in self.map.iter_mut() {
            direntry.need_update = true;
        }

    }
}
