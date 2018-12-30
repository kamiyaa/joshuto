extern crate fs_extra;

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

    pub fn insert(&mut self, dirlist: structs::JoshutoDirList)
    {
        self.map.insert(dirlist.path.clone(), dirlist);
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
        match self.map.remove(&path.to_path_buf()) {
            Some(mut dir_entry) => {

                if dir_entry.update_needed || dir_entry.need_update() {
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
            self.map.insert(s.path.clone(), s);
        }
    }


    pub fn depecrate_all_entries(&mut self)
    {
        self.map.iter_mut().for_each(|(_, v)| v.update_needed = true);
    }
}

enum FileOp {
    Cut,
    Copy,
}

pub struct FileClipboard {
    files: Vec<path::PathBuf>,
    fileop: FileOp,
}

impl FileClipboard {
    pub fn new() -> Self
    {
        FileClipboard {
            files: Vec::new(),
            fileop: FileOp::Copy,
        }
    }

    pub fn prepare(dirlist: &structs::JoshutoDirList)
            -> Option<Vec<path::PathBuf>>
    {
        if let Some(contents) = dirlist.contents.as_ref() {
            let selected: Vec<path::PathBuf> = contents.iter()
                    .filter(|entry| entry.selected)
                    .map(|entry| entry.entry.path()).collect();
            if selected.len() > 0 {
                Some(selected)
            } else if dirlist.index >= 0 {
                Some(vec![contents[dirlist.index as usize].entry.path()])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn prepare_cut(&mut self, dirlist: &structs::JoshutoDirList)
    {
        match FileClipboard::prepare(dirlist) {
            Some(s) => {
                self.files = s;
                self.fileop = FileOp::Cut;
            }
            None => {},
        }
    }

    pub fn cut(&mut self, destination: path::PathBuf, options: &fs_extra::dir::CopyOptions) {
        let mut destination = destination;
        let cut_options = fs_extra::file::CopyOptions {
                overwrite: options.overwrite,
                skip_exist: options.skip_exist,
                buffer_size: options.buffer_size,
            };
        for path in &self.files {
            if path.is_dir() {
                fs_extra::dir::move_dir(&path, &destination, options);
            } else {
                let mut comp = path.components().rev();
                destination.push(comp.next().unwrap());
                fs_extra::file::move_file(&path, &destination, &cut_options);
                destination.pop();
            }
        }
        self.files.clear();
    }

    pub fn prepare_copy(&mut self, dirlist: &structs::JoshutoDirList)
    {
        match FileClipboard::prepare(dirlist) {
            Some(s) => {
                self.files = s;
                self.fileop = FileOp::Copy;
            }
            None => {},
        }
    }

    pub fn copy(&mut self, destination: path::PathBuf, options: &fs_extra::dir::CopyOptions) {
        let mut destination = destination;
        let handle = |process_info: fs_extra::TransitProcess| {
            eprintln!("{}", process_info.copied_bytes);
            fs_extra::dir::TransitProcessResult::ContinueOrAbort
        };

        match fs_extra::copy_items_with_progress(&self.files, &destination, &options, handle)
        {
            Ok(s) => {},
            Err(e) => {},
        }
        self.files.clear();
    }

    pub fn paste(&mut self, destination: path::PathBuf, options: &fs_extra::dir::CopyOptions) {
        match self.fileop {
            FileOp::Copy => self.copy(destination, options),
            FileOp::Cut => self.cut(destination, options),
        }
    }
}

pub struct DeleteClipboard {
    files: Vec<path::PathBuf>,
}

impl DeleteClipboard {
    pub fn new() -> Self
    {
        DeleteClipboard {
            files: Vec::new(),
        }
    }

    pub fn prepare(&mut self, dirlist: &structs::JoshutoDirList)
    {
        match FileClipboard::prepare(dirlist) {
            Some(s) => {
                self.files = s;
            }
            None => {},
        }
    }

    pub fn execute(&mut self) -> std::io::Result<()>
    {
        for path in &self.files {
            if path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            } else {
                std::fs::remove_file(&path)?;
            }
        }
        self.files.clear();
        Ok(())
    }
}
