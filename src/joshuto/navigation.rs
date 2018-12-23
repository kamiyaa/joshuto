use std;
use std::path;

use joshuto::structs;
use joshuto::sort;
use joshuto::history;

pub fn set_dir_cursor_index(history : &mut history::History,
        curr_view: &mut structs::JoshutoDirList,
        preview_view: Option<structs::JoshutoDirList>,
        sort_type: &sort::SortType,
        new_index: i32)
        -> Result<structs::JoshutoDirList, std::io::Error>
{
    let curr_index = curr_view.index as usize;
    match preview_view {
        Some(s) => {
            let folder_path: path::PathBuf = curr_view.contents.as_ref()
                        .unwrap()[curr_index].entry.path();
            history.insert(folder_path, s);
        },
        None => {},
    };

    curr_view.index = new_index;
    let curr_index = curr_view.index as usize;
    let new_path: path::PathBuf = curr_view.contents.as_ref()
                                    .unwrap()[curr_index].entry.path();

    history.pop_or_create(new_path.as_path(), sort_type)
}

