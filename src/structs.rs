use std::ffi;
use std::fs;
use std::path::{Path, PathBuf};
use std::time;

use crate::sort;
use crate::window::JoshutoPageState;

#[derive(Clone, Debug)]
pub struct JoshutoMetadata {
    pub len: u64,
    pub modified: time::SystemTime,
    pub permissions: fs::Permissions,
    pub file_type: fs::FileType,
}

impl JoshutoMetadata {
    pub fn from(metadata: &fs::Metadata) -> Result<Self, std::io::Error> {
        let len = metadata.len();
        let modified = metadata.modified()?;
        let permissions = metadata.permissions();
        let file_type = metadata.file_type();

        Ok(JoshutoMetadata {
            len,
            modified,
            permissions,
            file_type,
        })
    }
}

#[derive(Clone)]
pub struct JoshutoDirEntry {
    pub file_name: ffi::OsString,
    pub file_name_as_string: String,
    pub path: PathBuf,
    pub metadata: JoshutoMetadata,
    pub selected: bool,
    pub marked: bool,
}

impl JoshutoDirEntry {
    pub fn from(direntry: &fs::DirEntry) -> Result<Self, std::io::Error> {
        let file_name = direntry.file_name();
        let file_name_as_string: String = match file_name.clone().into_string() {
            Ok(s) => s,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Failed to get file_name",
                ));
            }
        };
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

impl std::fmt::Debug for JoshutoDirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "JoshutoDirEntry {{\n\tfile_name: {:?}, \n\tfile_name_as_string: {}, \n\tpath: {:?} \n}}",
            self.file_name, self.file_name_as_string, self.path)
    }
}

#[derive(Debug)]
pub struct JoshutoDirList {
    pub index: Option<usize>,
    pub path: PathBuf,
    pub update_needed: bool,
    pub metadata: JoshutoMetadata,
    pub contents: Vec<JoshutoDirEntry>,
    pub pagestate: JoshutoPageState,
}

impl JoshutoDirList {
    pub fn new(path: PathBuf, sort_option: &sort::SortOption) -> Result<Self, std::io::Error> {
        let mut contents = Self::read_dir_list(path.as_path(), sort_option)?;
        contents.sort_by(&sort_option.compare_func());

        let index = if !contents.is_empty() { Some(0) } else { None };

        let metadata = fs::metadata(&path)?;
        let metadata = JoshutoMetadata::from(&metadata)?;
        let pagestate = JoshutoPageState::new();

        Ok(JoshutoDirList {
            index,
            path,
            update_needed: false,
            metadata,
            contents,
            pagestate,
        })
    }

    fn read_dir_list(
        path: &Path,
        sort_option: &sort::SortOption,
    ) -> Result<Vec<JoshutoDirEntry>, std::io::Error> {
        let filter_func = sort_option.filter_func();
        let results: fs::ReadDir = fs::read_dir(path)?;
        let result_vec: Vec<JoshutoDirEntry> = results
            .filter(filter_func)
            .filter_map(sort::map_entry_default)
            .collect();
        Ok(result_vec)
    }

    pub fn need_update(&self) -> bool {
        if self.update_needed {
            return true;
        }
        if let Ok(metadata) = std::fs::metadata(&self.path) {
            if let Ok(modified) = metadata.modified() {
                return self.metadata.modified < modified;
            }
        }
        true
    }

    pub fn update_contents(
        &mut self,
        sort_option: &sort::SortOption,
    ) -> Result<(), std::io::Error> {
        let sort_func = sort_option.compare_func();
        self.update_needed = false;

        let mut contents = Self::read_dir_list(&self.path, sort_option)?;
        contents.sort_by(&sort_func);

        let contents_len = contents.len();
        if contents_len == 0 {
            self.index = None;
        } else {
            self.index = match self.index {
                Some(index) => {
                    if index >= contents_len {
                        Some(contents_len - 1)
                    } else {
                        self.index
                    }
                }
                None => Some(0),
            };
        }

        let metadata = std::fs::metadata(&self.path)?;
        let metadata = JoshutoMetadata::from(&metadata)?;
        self.metadata = metadata;
        self.contents = contents;
        Ok(())
    }

    pub fn selected_entries<'a>(&'a self) -> impl Iterator<Item = &'a JoshutoDirEntry> {
        self.contents.iter().filter(|entry| entry.selected)
    }

    pub fn get_selected_paths(&self) -> Option<Vec<PathBuf>> {
        let vec: Vec<PathBuf> = self.selected_entries().map(|e| e.path.clone()).collect();
        if vec.is_empty() {
            Some(vec![self.get_curr_ref()?.path.clone()])
        } else {
            Some(vec)
        }
    }

    pub fn get_curr_ref(&self) -> Option<&JoshutoDirEntry> {
        self.get_curr_ref_(self.index?)
    }

    pub fn get_curr_mut(&mut self) -> Option<&mut JoshutoDirEntry> {
        self.get_curr_mut_(self.index?)
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
