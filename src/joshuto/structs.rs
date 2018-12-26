use std;
use std::fs;
use std::path;
use std::time;
use std::ffi;

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
    pub need_update: bool,
    pub modified: time::SystemTime,
    pub contents: Option<Vec<JoshutoDirEntry>>,
    pub selection: Vec<fs::DirEntry>,
}

impl JoshutoDirList {

    pub fn new(path: &path::Path, sort_type: &sort::SortType) -> Result<JoshutoDirList, std::io::Error>
    {
        let mut dir_contents = JoshutoDirList::read_dir_list(path, sort_type)?;

        dir_contents.sort_by(&sort_type.compare_func());

        let modified = std::fs::metadata(&path)?.modified()?;

        let index = if dir_contents.len() > 0 {
                0
            } else {
                -1
            };

        Ok(JoshutoDirList {
            index,
            path: path.clone().to_path_buf(),
            need_update: false,
            modified,
            contents: Some(dir_contents),
            selection: Vec::new(),
        })
    }

    pub fn update(&mut self, sort_type: &sort::SortType)
    {
        let sort_func = sort_type.compare_func();

        self.need_update = false;

        if let Ok(mut dir_contents) = JoshutoDirList::read_dir_list(&self.path, sort_type) {
            dir_contents.sort_by(&sort_func);

            if self.index >= 0 {
                let indexed_filename = match self.contents.as_ref() {
                    Some(s) => {
                        s[self.index as usize].entry.file_name()
                    },
                    None => ffi::OsString::from(""),
                };
                for (i, entry) in dir_contents.iter().enumerate() {
                    if indexed_filename == entry.entry.file_name() {
                        self.index = i as i32;
                        break;
                    }
                }
            }

            self.contents = Some(dir_contents);
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
        match self.contents {
            Some(ref s) => {
                if self.index >= 0 && (self.index as usize) < s.len() {
                    Some(&s[self.index as usize])
                } else {
                    None
                }
            },
            None => {
                None
            }
        }
    }
    pub fn get_dir_entry(&self, index: i32) -> Option<&JoshutoDirEntry>
    {
        match self.contents {
            Some(ref s) => {
                if index >= 0 && (index as usize) < s.len() {
                    Some(&s[index as usize])
                } else {
                    None
                }
            },
            None => {
                None
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
