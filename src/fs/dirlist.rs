use std::{fs, path};

use crate::fs::{JoshutoDirEntry, JoshutoMetadata};
use crate::sort::SortOption;

#[derive(Debug)]
pub struct JoshutoDirList {
    pub index: Option<usize>,
    path: path::PathBuf,
    content_outdated: bool,
    order_outdated: bool,
    pub metadata: JoshutoMetadata,
    pub contents: Vec<JoshutoDirEntry>,
}

impl JoshutoDirList {
    pub fn new(path: path::PathBuf, sort_option: &SortOption) -> std::io::Result<Self> {
        let filter_func = sort_option.filter_func();
        let mut contents = read_dir_list(path.as_path(), filter_func)?;
        let compare_func = sort_option.compare_func();
        contents.sort_by(compare_func);

        let index = if contents.is_empty() { None } else { Some(0) };

        let metadata = JoshutoMetadata::from(&path)?;

        Ok(JoshutoDirList {
            index,
            path,
            content_outdated: false,
            order_outdated: false,
            metadata,
            contents,
        })
    }

    pub fn sort<F>(&mut self, sort_func: F)
    where
        F: Fn(&JoshutoDirEntry, &JoshutoDirEntry) -> std::cmp::Ordering,
    {
        self.contents.sort_by(sort_func);
    }

    pub fn depreciate(&mut self) {
        self.content_outdated = true;
    }

    pub fn need_update(&self) -> bool {
        self.content_outdated
    }

    pub fn file_path(&self) -> &path::PathBuf {
        &self.path
    }

    pub fn reload_contents(&mut self, sort_option: &SortOption) -> std::io::Result<()> {
        let filter_func = sort_option.filter_func();
        let mut contents = read_dir_list(&self.path, filter_func)?;
        let sort_func = sort_option.compare_func();
        contents.sort_by(sort_func);

        let contents_len = contents.len();

        let index: Option<usize> = {
            if contents_len == 0 {
                None
            } else {
                match self.get_curr_ref() {
                    Some(entry) => contents
                        .iter()
                        .enumerate()
                        .find(|(i, e)| e.file_name() == entry.file_name())
                        .and_then(|(i, e)| Some(i))
                        .or(Some(0)),
                    None => Some(0),
                }
            }
        };

        let metadata = JoshutoMetadata::from(&self.path)?;
        self.metadata = metadata;
        self.contents = contents;
        self.index = index;
        self.content_outdated = false;

        Ok(())
    }

    pub fn selected_entries(&self) -> impl Iterator<Item = &JoshutoDirEntry> {
        self.contents.iter().filter(|entry| entry.is_selected())
    }

    pub fn get_selected_paths(&self) -> Vec<&path::PathBuf> {
        let vec: Vec<&path::PathBuf> = self.selected_entries().map(|e| e.file_path()).collect();
        if !vec.is_empty() {
            vec
        } else {
            match self.get_curr_ref() {
                Some(s) => vec![s.file_path()],
                _ => vec![],
            }
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

fn read_dir_list<F>(path: &path::Path, filter_func: F) -> std::io::Result<Vec<JoshutoDirEntry>>
where
    F: Fn(&Result<fs::DirEntry, std::io::Error>) -> bool,
{
    let results: Vec<JoshutoDirEntry> = fs::read_dir(path)?
        .filter(filter_func)
        .filter_map(map_entry_default)
        .collect();
    Ok(results)
}

fn map_entry_default(result: std::io::Result<fs::DirEntry>) -> Option<JoshutoDirEntry> {
    match result {
        Ok(direntry) => match JoshutoDirEntry::from(&direntry) {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
