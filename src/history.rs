use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::config::clean::app::display::dirlist::DirListDisplayOptions;
use crate::config::clean::app::display::tab::TabDisplayOption;
use crate::config::clean::app::display::DisplayOption;
use crate::context::UiContext;
use crate::fs::{JoshutoDirEntry, JoshutoDirList, JoshutoMetadata};

pub trait DirectoryHistory {
    fn populate_to_root(
        &mut self,
        path: &Path,
        ui_context: &UiContext,
        options: &DisplayOption,
        tab_options: &TabDisplayOption,
    ) -> io::Result<()>;
    fn create_or_soft_update(
        &mut self,
        path: &Path,
        options: &DisplayOption,
        tab_options: &TabDisplayOption,
    ) -> io::Result<()>;
    fn create_or_reload(
        &mut self,
        path: &Path,
        options: &DisplayOption,
        tab_options: &TabDisplayOption,
    ) -> io::Result<()>;
    fn reload(
        &mut self,
        path: &Path,
        options: &DisplayOption,
        tab_options: &TabDisplayOption,
    ) -> io::Result<()>;
    fn depreciate_all_entries(&mut self);

    fn depreciate_entry(&mut self, path: &Path);
}

pub type JoshutoHistory = HashMap<PathBuf, JoshutoDirList>;

impl DirectoryHistory for JoshutoHistory {
    fn populate_to_root(
        &mut self,
        path: &Path,
        ui_context: &UiContext,
        options: &DisplayOption,
        tab_options: &TabDisplayOption,
    ) -> io::Result<()> {
        let mut dirlists = Vec::new();

        let mut prev: Option<&Path> = None;
        for curr in path.ancestors() {
            if self.contains_key(curr) {
                let mut new_dirlist =
                    create_dirlist_with_history(self, curr, options, tab_options)?;
                if let Some(ancestor) = prev.as_ref() {
                    if let Some(i) = get_index_of_value(&new_dirlist.contents, ancestor) {
                        new_dirlist.set_index(Some(i), ui_context, options);
                    }
                }
                dirlists.push(new_dirlist);
            } else {
                let mut new_dirlist =
                    JoshutoDirList::from_path(curr.to_path_buf().clone(), options, tab_options)?;
                if let Some(ancestor) = prev.as_ref() {
                    if let Some(i) = get_index_of_value(&new_dirlist.contents, ancestor) {
                        new_dirlist.set_index(Some(i), ui_context, options);
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

    fn create_or_soft_update(
        &mut self,
        path: &Path,
        options: &DisplayOption,
        tab_options: &TabDisplayOption,
    ) -> io::Result<()> {
        let (contains_key, need_update) = if let Some(dirlist) = self.get(path) {
            (true, dirlist.need_update())
        } else {
            (false, true)
        };
        if need_update {
            let dirlist = if contains_key {
                create_dirlist_with_history(self, path, options, tab_options)?
            } else {
                JoshutoDirList::from_path(path.to_path_buf(), options, tab_options)?
            };
            self.insert(path.to_path_buf(), dirlist);
        }
        Ok(())
    }

    fn create_or_reload(
        &mut self,
        path: &Path,
        options: &DisplayOption,
        tab_options: &TabDisplayOption,
    ) -> io::Result<()> {
        let dirlist = if self.contains_key(path) {
            create_dirlist_with_history(self, path, options, tab_options)?
        } else {
            JoshutoDirList::from_path(path.to_path_buf(), options, tab_options)?
        };
        self.insert(path.to_path_buf(), dirlist);
        Ok(())
    }

    fn reload(
        &mut self,
        path: &Path,
        options: &DisplayOption,
        tab_options: &TabDisplayOption,
    ) -> io::Result<()> {
        let dirlist = create_dirlist_with_history(self, path, options, tab_options)?;
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
    tab_options: &TabDisplayOption,
) -> io::Result<JoshutoDirList> {
    let filter_func = options.filter_func();
    let mut contents = read_directory(path, filter_func, options, tab_options)?;

    // re-use directory size information on reload
    for entry in contents.iter_mut() {
        if entry.metadata.is_dir() {
            if let Some(lst) = history.get(entry.file_path()) {
                entry.metadata.update_directory_size(lst.len());
            }
        }
    }

    // preserve selection status of entries on reload
    if let Some(former_dir_list) = history.get(path) {
        let former_entries_by_file_name = HashMap::<&str, &JoshutoDirEntry>::from_iter(
            former_dir_list.contents.iter().map(|e| (e.file_name(), e)),
        );
        for entry in contents.iter_mut() {
            if let Some(former_entry) = former_entries_by_file_name.get(entry.file_name()) {
                entry.set_permanent_selected(former_entry.is_permanent_selected());
                entry.set_visual_mode_selected(former_entry.is_visual_mode_selected());
            }
        }
    }

    let sort_options = tab_options.sort_options_ref();
    contents.sort_by(|f1, f2| sort_options.compare(f1, f2));

    let contents_len = contents.len();
    let index = if contents_len == 0 {
        None
    } else {
        match history.get(path) {
            Some(dirlist) => match dirlist.get_index() {
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
    let viewport_index: usize = if contents_len == 0 {
        0
    } else {
        match history.get(path) {
            Some(dirlist) => match dirlist.first_index_for_viewport() {
                i if i >= contents_len => contents_len - 1,
                i => i,
            },
            None => 0,
        }
    };
    let visual_mode_anchor_index = history.get(path).and_then(|dirlist| {
        dirlist
            .get_visual_mode_anchor_index()
            .map(|old_visual_mode_anchor_index| {
                if old_visual_mode_anchor_index < contents_len {
                    old_visual_mode_anchor_index
                } else {
                    contents_len - 1
                }
            })
    });

    let metadata = JoshutoMetadata::from(path)?;
    let dirlist = JoshutoDirList::new(
        path.to_path_buf(),
        contents,
        index,
        viewport_index,
        visual_mode_anchor_index,
        metadata,
    );

    Ok(dirlist)
}

pub fn read_directory<F>(
    path: &Path,
    filter_func: F,
    options: &DisplayOption,
    tab_options: &TabDisplayOption,
) -> io::Result<Vec<JoshutoDirEntry>>
where
    F: Fn(&walkdir::DirEntry, &DisplayOption, &DirListDisplayOptions) -> bool,
{
    let dirlist_opts = tab_options
        .dirlist_options_ref(&path.to_path_buf())
        .map(|v| v.to_owned())
        .unwrap_or_default();

    let results: Vec<JoshutoDirEntry> = WalkDir::new(path)
        .max_depth(dirlist_opts.depth() as usize + 1)
        .into_iter()
        .filter_entry(|e| {
            if e.path().to_str().cmp(&path.to_str()).is_ne() {
                filter_func(e, options, &dirlist_opts)
            } else {
                true
            }
        })
        .filter(|e| {
            if let Ok(e) = e.as_ref() {
                e.path().to_str().cmp(&path.to_str()).is_ne()
            } else {
                true
            }
        })
        .filter_map(|res| JoshutoDirEntry::from(&res.ok()?, path, options).ok())
        .collect();

    Ok(results)
}
