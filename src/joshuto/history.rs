
use std;
use std::collections::HashMap;
use std::fs;
use std::path;

use joshuto::structs;
use joshuto::sort;

pub struct DirHistory {
    map: HashMap<path::PathBuf, structs::JoshutoDirList>,
}

impl DirHistory {

    pub fn new() -> Self
    {
        DirHistory {
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
                match structs::JoshutoDirList::new(parent.clone(), sort_type) {
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
                    dir_entry.update(&sort_type);
                }
                Ok(dir_entry)
            },
            None => {
                structs::JoshutoDirList::new(path.clone().to_path_buf(), &sort_type)
            }
        }
    }

    pub fn put_back(&mut self, dirlist: Option<structs::JoshutoDirList>)
    {
        if let Some(s) = dirlist {
            let path: path::PathBuf = s.path.clone();
            self.map.insert(path, s);
        }
    }


    pub fn depecrate_all_entries(&mut self)
    {
        self.map.iter_mut().for_each(|(_, v)| v.need_update = true);
    }
}

pub struct FileClipboard {
    files: Vec<path::PathBuf>,
}

impl FileClipboard {

    pub fn new() -> Self
    {
        FileClipboard {
            files: Vec::new(),
        }
    }

    pub fn copy()
    {
        
    }
}
