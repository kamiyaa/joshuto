use std::{fs, path};

use crate::sort;
use crate::structs::{JoshutoDirEntry, JoshutoMetadata};
use crate::ui;
use crate::window::JoshutoPageState;

#[derive(Debug)]
pub struct JoshutoDirList {
    pub index: Option<usize>,
    pub path: path::PathBuf,
    pub metadata: JoshutoMetadata,
    pub contents: Vec<JoshutoDirEntry>,
    pub pagestate: JoshutoPageState,
    outdated: bool,
}

impl JoshutoDirList {
    pub fn new(
        path: path::PathBuf,
        sort_option: &sort::SortOption,
    ) -> Result<Self, std::io::Error> {
        let mut contents = read_dir_list(path.as_path(), sort_option)?;
        contents.sort_by(&sort_option.compare_func());

        let index = if contents.is_empty() { None } else { Some(0) };

        let contents_len = contents.len();
        let (rows, _) = ui::getmaxyx();
        let end = if rows < 2 {
            0
        } else if contents_len > rows as usize - 2 {
            rows as usize - 2
        } else {
            contents_len
        };

        let metadata = JoshutoMetadata::from(&path)?;
        let pagestate = JoshutoPageState::new(end);

        Ok(JoshutoDirList {
            index,
            path,
            outdated: false,
            metadata,
            contents,
            pagestate,
        })
    }

    pub fn depreciate(&mut self) {
        self.outdated = true;
    }

    pub fn need_update(&self) -> bool {
        self.outdated
    }

    pub fn update_contents(
        &mut self,
        sort_option: &sort::SortOption,
    ) -> Result<(), std::io::Error> {
        self.outdated = false;

        let sort_func = sort_option.compare_func();
        let mut contents = read_dir_list(&self.path, sort_option)?;
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

        let metadata = JoshutoMetadata::from(&self.path)?;
        self.metadata = metadata;
        self.contents = contents;
        Ok(())
    }

    pub fn selected_entries(&self) -> impl Iterator<Item = &JoshutoDirEntry> {
        self.contents.iter().filter(|entry| entry.selected)
    }

    pub fn get_selected_paths(&self) -> Option<Vec<path::PathBuf>> {
        let vec: Vec<path::PathBuf> = self.selected_entries().map(|e| e.path.clone()).collect();
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

fn read_dir_list(
    path: &path::Path,
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
