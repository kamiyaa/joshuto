use std::io;
use std::path::Path;

use crate::config::option::DisplayOption;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::fs::{JoshutoDirEntry, JoshutoDirList, JoshutoMetadata};

use walkdir::WalkDir;

fn _is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn _walk_directory(
    path: &Path,
    options: &DisplayOption,
    depth: usize,
) -> io::Result<Vec<JoshutoDirEntry>> {
    let results: Vec<JoshutoDirEntry> = WalkDir::new(path)
        .max_depth(depth)
        .into_iter()
        .filter_entry(|e| {
            if options.show_hidden() {
                true
            } else {
                !_is_hidden(e)
            }
        })
        .filter(|e| {
            if let Ok(e) = e.as_ref() {
                e.path().to_str().cmp(&path.to_str()).is_ne()
            } else {
                true
            }
        })
        .filter_map(|res| JoshutoDirEntry::from_walk(&res.ok()?, path, options).ok())
        .collect();

    Ok(results)
}

pub fn flatten(depth: usize, context: &mut AppContext) -> JoshutoResult {
    let path = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();

    let options = context.config_ref().display_options_ref().clone();
    let tab_options = context
        .tab_context_ref()
        .curr_tab_ref()
        .option_ref()
        .clone();

    let mut index: Option<usize> = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|lst| lst.get_index())
        .unwrap_or(None);
    let viewport_index: usize = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|lst| lst.first_index_for_viewport())
        .unwrap_or(0);

    let mut contents = _walk_directory(path.as_path(), &options, depth)?;
    let history = context.tab_context_mut().curr_tab_mut().history_mut();

    if contents.is_empty() {
        index = None;
    }

    let sort_options = tab_options.sort_options_ref();

    contents.sort_by(|f1, f2| sort_options.compare(f1, f2));

    let metadata = JoshutoMetadata::from(path.as_path())?;
    let dirlist = JoshutoDirList::new(path.clone(), contents, index, viewport_index, metadata);
    history.insert(path, dirlist);

    Ok(())
}
