use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::config::{AppDirListDisplayOptions, DisplayOption, TabDisplayOption};
use crate::fs::{AppDirEntry, AppDirectoryList};
use crate::state::UiState;

/// Operations on a cache of previously-visited directory listings, keyed by path.
pub trait DirectoryHistory {
    /// Inserts or replaces cached listings, keyed by each listing's own path.
    fn insert_entries(&mut self, entries: Vec<AppDirectoryList>);
    /// Marks every cached listing as stale, forcing a re-read on next access.
    fn depreciate_all_entries(&mut self);
}

/// Cache of previously-read directory listings, keyed by directory path.
pub type AppDirectoryHistory = HashMap<PathBuf, AppDirectoryList>;

impl DirectoryHistory for AppDirectoryHistory {
    fn insert_entries(&mut self, entries: Vec<AppDirectoryList>) {
        for dirlist in entries {
            self.insert(dirlist.file_path().to_path_buf(), dirlist);
        }
    }

    fn depreciate_all_entries(&mut self) {
        self.iter_mut().for_each(|(_, v)| v.depreciate());
    }
}

fn get_index_of_value(arr: &[AppDirEntry], val: &Path) -> Option<usize> {
    arr.iter().enumerate().find_map(|(i, dir)| {
        if dir.file_path() == val {
            Some(i)
        } else {
            None
        }
    })
}

/// Re-reads `path` from disk, but reuses cached directory sizes and preserves selection state
/// and cursor position from `history` where the underlying entries still match.
pub fn create_dirlist_with_history(
    history: &AppDirectoryHistory,
    path: &Path,
    options: &DisplayOption,
    tab_options: &TabDisplayOption,
) -> io::Result<AppDirectoryList> {
    let dirlist = AppDirectoryList::from_path(path.to_path_buf(), options, tab_options)?;
    Ok(merge_dirlist_with_history(history, dirlist, tab_options))
}

/// Carries cached state from `history` over to a freshly read `dirlist`: directory sizes,
/// selection status, cumulative sizes, cursor, viewport, and visual-mode anchor. Does no IO,
/// so it's safe to call on the main thread with a listing read in the background.
pub fn merge_dirlist_with_history(
    history: &AppDirectoryHistory,
    mut dirlist: AppDirectoryList,
    tab_options: &TabDisplayOption,
) -> AppDirectoryList {
    let path = dirlist.path.as_path();
    let contents = &mut dirlist.contents;

    // re-use directory size information on reload
    for entry in contents.iter_mut() {
        if entry.metadata.is_dir() {
            if let Some(lst) = history.get(entry.file_path()) {
                entry.metadata.update_directory_size(lst.len());
            }
        }
    }

    let former_dir_list = history.get(path);

    // preserve selection status of entries on reload
    if let Some(former_dir_list) = former_dir_list {
        let former_entries_by_file_name = HashMap::<&str, &AppDirEntry>::from_iter(
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

    // sizes may have changed, which matters when sorting by size
    let sort_options = &tab_options.sort_options;
    contents.sort_by(|f1, f2| sort_options.compare(f1, f2));

    let contents_len = contents.len();
    let index = if contents_len == 0 {
        None
    } else {
        match former_dir_list {
            Some(former) => match former.get_index() {
                Some(i) if i >= contents_len => Some(contents_len - 1),
                Some(i) => {
                    let entry = &former.contents[i];
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
        match former_dir_list {
            Some(former) => match former.first_index_for_viewport() {
                i if i >= contents_len => contents_len - 1,
                i => i,
            },
            None => 0,
        }
    };
    let visual_mode_anchor_index = if contents_len == 0 {
        None
    } else {
        former_dir_list
            .and_then(|former| former.get_visual_mode_anchor_index())
            .map(|i| i.min(contents_len - 1))
    };

    dirlist.index = index;
    dirlist.viewport_index = viewport_index;
    dirlist.visual_mode_anchor_index = visual_mode_anchor_index;
    dirlist.need_update = false;
    dirlist
}

/// Reads the immediate (or, if flattened, nested) contents of `path` from disk, keeping only
/// entries that pass `filter_func`.
pub fn read_directory<F>(
    path: &Path,
    filter_func: F,
    display_options: &DisplayOption,
    tab_options: &TabDisplayOption,
) -> io::Result<Vec<AppDirEntry>>
where
    F: Fn(&walkdir::DirEntry, &DisplayOption, &AppDirListDisplayOptions) -> bool,
{
    let dirlist_opts = tab_options
        .dirlist_options
        .get(path)
        .map(|v| v.to_owned())
        .unwrap_or_default();

    let results: Vec<AppDirEntry> = WalkDir::new(path)
        .max_depth(dirlist_opts.depth() as usize + 1)
        .into_iter()
        // depth 0 is `path` itself
        .filter_entry(|e| e.depth() == 0 || filter_func(e, display_options, &dirlist_opts))
        .filter(|e| e.as_ref().map(|e| e.depth() != 0).unwrap_or(true))
        .filter_map(|res| AppDirEntry::from(&res.ok()?, path, display_options).ok())
        .collect();

    Ok(results)
}

/// Builds a directory listing for `path` and each of its ancestors up to the filesystem root,
/// with each listing's cursor set to the child it descended through, reusing `history` where
/// possible.
pub fn generate_entries_to_root(
    path: &Path,
    history: &AppDirectoryHistory,
    ui_state: &UiState,
    display_options: &DisplayOption,
    tab_options: &TabDisplayOption,
) -> io::Result<Vec<AppDirectoryList>> {
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
            let mut new_dirlist = AppDirectoryList::from_path(
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
    use crate::config::{DisplayOption, TabDisplayOption};
    use crate::fs::{AppDirectoryHistory, AppDirectoryList, DirectoryHistory};

    use super::merge_dirlist_with_history;

    #[test]
    fn merge_keeps_cursor_and_selection_when_entries_are_added() {
        let dir = std::env::temp_dir().join(format!("joshuto-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        for name in ["b", "c"] {
            std::fs::write(dir.join(name), "").unwrap();
        }
        let options = DisplayOption::default();
        let tab_options = TabDisplayOption::default();

        let mut old = AppDirectoryList::from_path(dir.clone(), &options, &tab_options).unwrap();
        let b = old
            .contents
            .iter()
            .position(|e| e.file_name() == "b")
            .unwrap();
        let c = old
            .contents
            .iter()
            .position(|e| e.file_name() == "c")
            .unwrap();
        old.index = Some(b);
        old.contents[c].set_permanent_selected(true);
        let mut history = AppDirectoryHistory::new();
        history.insert_entries(vec![old]);

        // a new entry sorts before the cursor, shifting every index
        std::fs::write(dir.join("a"), "").unwrap();
        let fresh = AppDirectoryList::from_path(dir.clone(), &options, &tab_options).unwrap();
        let merged = merge_dirlist_with_history(&history, fresh, &tab_options);

        let cursor = merged.index.map(|i| merged.contents[i].file_name());
        assert_eq!(cursor, Some("b"));
        let selected: Vec<_> = merged.iter_selected().map(|e| e.file_name()).collect();
        assert_eq!(selected, ["c"]);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
