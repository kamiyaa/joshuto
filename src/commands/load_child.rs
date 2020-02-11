use std::path::PathBuf;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::ui::TuiBackend;

pub struct LoadChild {}

impl LoadChild {
    pub fn load_child(context: &mut JoshutoContext, backend: &mut TuiBackend) {
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        let mut path: Option<PathBuf> = None;

        if let Some(curr_list) = curr_tab.curr_list_ref() {
            if let Some(index) = curr_list.index {
                let entry = &curr_list.contents[index];
                path = Some(entry.file_path().clone())
            }
        }

        // get preview
        if let Some(path) = path {
            if path.is_dir() {
                curr_tab
                    .history
                    .create_or_update(path.as_path(), &context.config_t.sort_option);
            }
        }
    }
}
