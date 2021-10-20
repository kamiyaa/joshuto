use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::config::option::DisplayOption;
use crate::fs::{JoshutoDirEntry, JoshutoDirList, JoshutoMetadata};

pub trait DirectoryHistory {
    fn populate_to_root(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn create_or_soft_update(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn create_or_reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn depreciate_all_entries(&mut self);

    fn depreciate_entry(&mut self, path: &Path);
}

pub type JoshutoHistory = HashMap<PathBuf, JoshutoDirList>;

impl DirectoryHistory for JoshutoHistory {
    fn populate_to_root(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let mut dirlists = Vec::new();

        let mut prev: Option<&Path> = None;
        for curr in path.ancestors() {
            if self.contains_key(curr) {
                let mut new_dirlist = create_dirlist_with_history(self, curr, options)?;
                if let Some(ancestor) = prev.as_ref() {
                    if let Some(i) = get_index_of_value(&new_dirlist.contents, ancestor) {
                        new_dirlist.index = Some(i);
                    }
                }
                dirlists.push(new_dirlist);
            } else {
                let mut new_dirlist =
                    JoshutoDirList::from_path(curr.to_path_buf().clone(), options)?;
                if let Some(ancestor) = prev.as_ref() {
                    if let Some(i) = get_index_of_value(&new_dirlist.contents, ancestor) {
                        new_dirlist.index = Some(i);
                    }
                }
                dirlists.push(new_dirlist);
            }
            prev = Some(curr);
        }
        for dirlist in dirlists {
            self.insert(dirlist.file_path().to_path_buf(), dirlist);
        }
        Ok(())
    }

    fn create_or_soft_update(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let (contains_key, need_update) = if let Some(dirlist) = self.get(path) {
            (true, dirlist.need_update())
        } else {
            (false, true)
        };
        if need_update {
            let dirlist = if contains_key {
                create_dirlist_with_history(self, path, options)?
            } else {
                JoshutoDirList::from_path(path.to_path_buf(), options)?
            };
            self.insert(path.to_path_buf(), dirlist);
        }
        Ok(())
    }

    fn create_or_reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let dirlist = if self.contains_key(path) {
            create_dirlist_with_history(self, path, options)?
        } else {
            JoshutoDirList::from_path(path.to_path_buf(), options)?
        };
        self.insert(path.to_path_buf(), dirlist);
        Ok(())
    }

    fn reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let dirlist = create_dirlist_with_history(self, path, options)?;
        self.insert(path.to_path_buf(), dirlist);
        Ok(())
    }

    fn depreciate_all_entries(&mut self) {
        self.iter_mut().for_each(|(_, v)| v.depreciate());
    }

    fn depreciate_entry(&mut self, path: &Path) {
        if let Some(v) = self.get_mut(path) {
            v.depreciate();
        }
    }
}

fn get_index_of_value(arr: &[JoshutoDirEntry], val: &Path) -> Option<usize> {
    arr.iter().enumerate().find_map(|(i, dir)| {
        if dir.file_path() == val {
            Some(i)
        } else {
            None
        }
    })
}

pub fn create_dirlist_with_history(
    history: &JoshutoHistory,
    path: &Path,
    options: &DisplayOption,
) -> io::Result<JoshutoDirList> {
    let filter_func = options.filter_func();
    let mut contents = read_directory(path, filter_func, options)?;
    for entry in contents.iter_mut() {
        if entry.metadata.is_dir() {
            if let Some(lst) = history.get(entry.file_path()) {
                entry.metadata.update_directory_size(lst.len());
            }
        }
    }

    let sort_options = options.sort_options_ref();
    contents.sort_by(|f1, f2| sort_options.compare(f1, f2));

    let contents_len = contents.len();
    let index: Option<usize> = if contents_len == 0 {
        None
    } else {
        match history.get(path) {
            Some(dirlist) => match dirlist.index {
                Some(i) if i >= contents_len => Some(contents_len - 1),
                Some(i) => {
                    let entry = &dirlist.contents[i];
                    contents
                        .iter()
                        .enumerate()
                        .find(|(_, e)| e.file_name() == entry.file_name())
                        .map(|(i, _)| i)
                        .or(Some(i))
                }
                None => Some(0),
            },
            None => Some(0),
        }
    };

    let metadata = JoshutoMetadata::from(path)?;
    let dirlist = JoshutoDirList::new(path.to_path_buf(), contents, index, metadata);

    Ok(dirlist)
}

pub fn read_directory<F>(
    path: &Path,
    filter_func: F,
    options: &DisplayOption,
) -> io::Result<Vec<JoshutoDirEntry>>
where
    F: Fn(&Result<fs::DirEntry, io::Error>) -> bool,
{
    let results: Vec<JoshutoDirEntry> = fs::read_dir(path)?
        .filter(filter_func)
        .filter_map(|res| JoshutoDirEntry::from(&res.ok()?, options).ok())
        .collect();

    Ok(results)
}
