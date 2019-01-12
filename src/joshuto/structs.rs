use std;
use std::fs;
use std::ffi;
use std::path;
use std::time;

use joshuto::sort;

#[derive(Debug)]
pub struct JoshutoMetadata {
    pub len: u64,
    pub modified: time::SystemTime,
    pub permissions: fs::Permissions,
    pub file_type: fs::FileType,
}

impl JoshutoMetadata {
    pub fn from(metadata: &fs::Metadata) -> Result<Self, std::io::Error>
    {
        let len = metadata.len();
        let modified = metadata.modified()?;
        let permissions = metadata.permissions();
        let file_type = metadata.file_type();

        Ok(JoshutoMetadata {
            len,
            modified,
            permissions,
            file_type
        })
    }
}

#[derive(Debug)]
pub struct JoshutoDirEntry {
    pub file_name: ffi::OsString,
    pub file_name_as_string: String,
    pub path: path::PathBuf,
    pub metadata: JoshutoMetadata,
    pub selected: bool,
    pub marked: bool,
}

impl JoshutoDirEntry {

    pub fn from(direntry: &fs::DirEntry) -> Result<Self, std::io::Error>
    {
        let file_name = direntry.file_name();
        let file_name_as_string: String = file_name.clone().into_string().unwrap();
        let path = direntry.path();

        let metadata = direntry.metadata()?;
        let metadata = JoshutoMetadata::from(&metadata)?;

        let dir_entry = JoshutoDirEntry {
            file_name,
            file_name_as_string,
            path,
            metadata,
            selected: false,
            marked: false,
        };
        Ok(dir_entry)
    }

}

#[derive(Debug)]
pub struct JoshutoDirList {
    pub index: i32,
    pub path: path::PathBuf,
    pub update_needed: bool,
    pub metadata: JoshutoMetadata,
    pub contents: Vec<JoshutoDirEntry>,
    pub selected: usize
}

impl JoshutoDirList {
    pub fn new(path: path::PathBuf, sort_type: &sort::SortType) -> Result<Self, std::io::Error>
    {
        let mut contents = Self::read_dir_list(path.as_path(), sort_type)?;
        contents.sort_by(&sort_type.compare_func());

        let index = if contents.len() > 0 {
                0
            } else {
                -1
            };

        let metadata = fs::metadata(&path)?;
        let metadata = JoshutoMetadata::from(&metadata)?;

        Ok(JoshutoDirList {
            index,
            path,
            update_needed: false,
            metadata,
            contents,
            selected: 0,
        })
    }

    pub fn need_update(&self) -> bool
    {
        if self.update_needed {
            return true;
        }
        if let Ok(metadata) = std::fs::metadata(&self.path) {
            if let Ok(modified) = metadata.modified() {
                return self.metadata.modified < modified;
            }
        }
        return true;
    }

    pub fn update(&mut self, sort_type: &sort::SortType)
    {
        let sort_func = sort_type.compare_func();

        self.update_needed = false;

        if let Ok(mut dir_contents) = Self::read_dir_list(&self.path, sort_type) {
            dir_contents.sort_by(&sort_func);

            if dir_contents.len() == 0 {
                self.index = -1;
            } else if self.index >= dir_contents.len() as i32 {
                self.index = dir_contents.len() as i32 - 1;
            } else if self.index >= 0 && (self.index as usize) < self.contents.len() {
                let index = self.index;
                let curr_file_name = &self.contents[index as usize].file_name;

                for (i, entry) in dir_contents.iter().enumerate() {
                    if *curr_file_name == entry.file_name {
                        self.index = i as i32;
                        break;
                    }
                }
            } else {
                self.index = 0;
            }
            self.contents = dir_contents;
        }

        if let Ok(metadata) = std::fs::metadata(&self.path) {
            if let Ok(metadata) = JoshutoMetadata::from(&metadata) {
                self.metadata = metadata;
            }
        }
    }

    fn read_dir_list(path : &path::Path, sort_type: &sort::SortType)
            -> Result<Vec<JoshutoDirEntry>, std::io::Error>
    {
        let filter_func = sort_type.filter_func();

        let results = fs::read_dir(path)?;
        let result_vec : Vec<JoshutoDirEntry> = results
                .filter_map(filter_func)
                .collect();
        Ok(result_vec)
    }

    pub fn get_curr_entry(&self) -> Option<&JoshutoDirEntry>
    {
        self.get_dir_entry(self.index)
    }

    fn get_dir_entry(&self, index: i32) -> Option<&JoshutoDirEntry>
    {
        if index >= 0 && (index as usize) < self.contents.len() {
            Some(&self.contents[index as usize])
        } else {
            None
        }
    }

    pub fn curr_toggle_select(&mut self)
    {
        let index = self.index;
        self.toggle_select(index);
    }

    fn toggle_select(&mut self, index: i32) {
        if index >= 0 && (index as usize) < self.contents.len() {
            let tmp_bool = !self.contents[index as usize].selected;
            self.contents[index as usize].selected = tmp_bool;
            if tmp_bool {
                self.selected = self.selected + 1;
            } else {
                self.selected = self.selected - 1;
            }
        }
    }
}
