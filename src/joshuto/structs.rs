use std;
use std::fs;
use std::path;
use std::time;

use joshuto::sort;
use joshuto::ui;
use joshuto::window;

#[derive(Debug)]
pub struct JoshutoDirEntry {
    pub entry: fs::DirEntry,
    pub selected: bool,
    pub marked: bool,
}

#[derive(Debug)]
pub struct JoshutoDirList {
    pub index: i32,
    pub path: path::PathBuf,
    pub update_needed: bool,
    pub modified: time::SystemTime,
    pub contents: Vec<JoshutoDirEntry>,
    pub selected: usize
}

impl JoshutoDirList {

    pub fn new(path: path::PathBuf, sort_type: &sort::SortType) -> Result<JoshutoDirList, std::io::Error>
    {
        let mut contents = Self::read_dir_list(path.as_path(), sort_type)?;

        contents.sort_by(&sort_type.compare_func());

        let modified = std::fs::metadata(&path)?.modified()?;

        let index = if contents.len() > 0 {
                0
            } else {
                -1
            };

        Ok(JoshutoDirList {
            index,
            path,
            update_needed: false,
            modified,
            contents,
            selected: 0,
        })
    }

    pub fn need_update(&self) -> bool
    {
        if let Ok(metadata) = std::fs::metadata(&self.path) {
            if let Ok(modified) = metadata.modified() {
                return self.modified < modified;
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

            if self.index >= dir_contents.len() as i32 {
                self.index = self.index - 1;
            } else if self.index >= 0 && dir_contents.len() > 0 {
                let indexed_filename = self.contents[self.index as usize].entry.file_name();
                for (i, entry) in dir_contents.iter().enumerate() {
                    if indexed_filename == entry.entry.file_name() {
                        self.index = i as i32;
                        break;
                    }
                }
            } else if dir_contents.len() > 0 {
                self.index = 0;
            } else {
                self.index = -1;
            }
            self.contents = dir_contents;
        }

        if let Ok(metadata) = std::fs::metadata(&self.path) {
            match metadata.modified() {
                Ok(s) => { self.modified = s; },
                Err(e) => { eprintln!("{}", e); },
            };
        }
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

    pub fn mark_curr_toggle(&mut self)
    {
        let index = self.index;
        self.mark_toggle(index);
    }

    fn mark_toggle(&mut self, index: i32) {
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

    pub fn display_contents(&self, win: &window::JoshutoPanel)
    {
        ui::display_contents(win, self);
    }

    fn read_dir_list(path : &path::Path, sort_type: &sort::SortType)
            -> Result<Vec<JoshutoDirEntry>, std::io::Error>
    {
        let filter_func = sort_type.filter_func();

        match fs::read_dir(path) {
            Ok(results) => {
                let mut result_vec : Vec<JoshutoDirEntry> = results
                        .filter_map(filter_func)
                        .collect();
                Ok(result_vec)
            },
            Err(e) => {
                Err(e)
            },
        }
    }
}

