use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::fs::{DirListDisplayOptions, JoshutoDirEntry, JoshutoDirList, JoshutoMetadata};
use crate::tab::TabDisplayOption;
use crate::types::option::display::DisplayOption;
use crate::types::state::UiState;

pub trait DirectoryHistory {
    fn insert_entries(&mut self, entries: Vec<JoshutoDirList>);
    fn depreciate_all_entries(&mut self);
}

pub type JoshutoHistory = HashMap<PathBuf, JoshutoDirList>;

impl DirectoryHistory for JoshutoHistory {
    fn insert_entries(&mut self, entries: Vec<JoshutoDirList>) {
        for dirlist in entries {
            self.insert(dirlist.file_path().to_path_buf(), dirlist);
        }
    }

    fn depreciate_all_entries(&mut self) {
        self.iter_mut().for_each(|(_, v)| v.depreciate());
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
                if let Some(size) = former_entry.metadata.cumulative_size() {
                    entry.metadata.update_cumulative_size(size);
                }
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
                // `i` indexes the PREVIOUS listing, and nothing keeps a stored
                // index in step with its own contents (`set_index` does not
                // bound it), so read it rather than index into it. The name
                // lookup is only a hint anyway; without it `i` still lands in
                // range, having passed the arm above.
                Some(i) => dirlist
                    .contents
                    .get(i)
                    .and_then(|entry| {
                        contents
                            .iter()
                            .position(|e| e.file_name() == entry.file_name())
                    })
                    .or(Some(i)),
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
    // An empty listing has nowhere for an anchor to point, and `contents_len - 1`
    // on nothing is a subtract-with-overflow panic. A directory whose entries are
    // all filtered out reaches exactly that: toggle hidden files off over a
    // directory holding only dotfiles while visual mode is on.
    let visual_mode_anchor_index = if contents_len == 0 {
        None
    } else {
        history.get(path).and_then(|dirlist| {
            dirlist
                .get_visual_mode_anchor_index()
                .map(|old_visual_mode_anchor_index| {
                    old_visual_mode_anchor_index.min(contents_len - 1)
                })
        })
    };

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
    display_options: &DisplayOption,
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
                filter_func(e, display_options, &dirlist_opts)
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
        .filter_map(|res| JoshutoDirEntry::from(&res.ok()?, path, display_options).ok())
        .collect();

    Ok(results)
}

pub fn generate_entries_to_root(
    path: &Path,
    history: &JoshutoHistory,
    ui_state: &UiState,
    display_options: &DisplayOption,
    tab_options: &TabDisplayOption,
) -> io::Result<Vec<JoshutoDirList>> {
    let mut dirlists = Vec::new();

    let mut prev: Option<&Path> = None;
    for curr in path.ancestors() {
        if history.contains_key(curr) {
            let mut new_dirlist =
                create_dirlist_with_history(history, curr, display_options, tab_options)?;
            if let Some(ancestor) = prev.as_ref() {
                if let Some(i) = get_index_of_value(&new_dirlist.contents, ancestor) {
                    new_dirlist.set_index(Some(i), ui_state, display_options);
                }
            }
            dirlists.push(new_dirlist);
        } else {
            let mut new_dirlist = JoshutoDirList::from_path(
                curr.to_path_buf().clone(),
                display_options,
                tab_options,
            )?;
            if let Some(ancestor) = prev.as_ref() {
                if let Some(i) = get_index_of_value(&new_dirlist.contents, ancestor) {
                    new_dirlist.set_index(Some(i), ui_state, display_options);
                }
            }
            dirlists.push(new_dirlist);
        }
        prev = Some(curr);
    }
    Ok(dirlists)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn scratch_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("joshuto-history-{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn options(show_hidden: bool) -> DisplayOption {
        let mut options = DisplayOption::default();
        options.show_hidden = show_hidden;
        options
    }

    /// A directory that filters down to nothing still has to produce a list.
    #[test]
    fn a_listing_that_filters_to_empty_keeps_its_visual_anchor_in_range() {
        let dir = scratch_dir("empty-after-filter");
        fs::write(dir.join(".hidden"), b"x").unwrap();
        let tab_options = TabDisplayOption::default();

        // Seen once with hidden files shown, and left in visual mode.
        let mut history = JoshutoHistory::new();
        let mut shown =
            create_dirlist_with_history(&history, &dir, &options(true), &tab_options).unwrap();
        assert_eq!(1, shown.len());
        shown.visual_mode_anchor_index = shown.get_index();
        history.insert_entries(vec![shown]);

        // Hiding them empties the listing. The anchor has nowhere to point.
        let hidden =
            create_dirlist_with_history(&history, &dir, &options(false), &tab_options).unwrap();
        assert!(hidden.is_empty());
        assert_eq!(None, hidden.get_index());
        assert_eq!(None, hidden.get_visual_mode_anchor_index());
        assert_eq!(0, hidden.first_index_for_viewport());

        let _ = fs::remove_dir_all(&dir);
    }

    /// A stored index is not bounded by its own contents, so read it rather
    /// than index into it.
    #[test]
    fn a_stored_index_past_its_own_contents_does_not_panic() {
        let dir = scratch_dir("stale-index");
        for name in ["a", "b", "c"] {
            fs::write(dir.join(name), b"x").unwrap();
        }
        let tab_options = TabDisplayOption::default();
        let mut history = JoshutoHistory::new();

        let seen =
            create_dirlist_with_history(&history, &dir, &options(false), &tab_options).unwrap();
        let metadata = JoshutoMetadata::from(&dir).unwrap();
        history.insert_entries(vec![JoshutoDirList::new(
            dir.clone(),
            Vec::new(),
            Some(seen.len() - 1),
            0,
            None,
            metadata,
        )]);

        let rebuilt =
            create_dirlist_with_history(&history, &dir, &options(false), &tab_options).unwrap();
        assert_eq!(3, rebuilt.len());
        assert_eq!(Some(2), rebuilt.get_index());

        let _ = fs::remove_dir_all(&dir);
    }
}
