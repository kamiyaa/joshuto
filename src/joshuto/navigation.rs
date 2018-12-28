use std;
use std::path;

use joshuto::structs;
use joshuto::sort;
use joshuto::history;

pub fn set_dir_cursor_index(history: &mut history::DirHistory,
        curr_view: &mut structs::JoshutoDirList,
        preview_view: Option<structs::JoshutoDirList>,
        sort_type: &sort::SortType,
        new_index: i32)
        -> Result<Option<structs::JoshutoDirList>, std::io::Error>
{
    let curr_index = curr_view.index as usize;
    if let Some(s) = preview_view {
        history.insert(s);
    }

    curr_view.index = new_index;
    let curr_index = curr_view.index as usize;
    let new_path: path::PathBuf = curr_view.contents.as_ref()
                                    .unwrap()[curr_index].entry.path();
    if new_path.is_dir() {
        match history.pop_or_create(new_path.as_path(), sort_type) {
            Ok(s) => Ok(Some(s)),
            Err(e) => Err(e),
        }
    } else {
        Ok(None)
    }
}

